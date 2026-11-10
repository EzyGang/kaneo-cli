use chrono::{DateTime, FixedOffset, Local, Utc};

use crate::api::types::time_entry::{TimeEntry, TimeEntryBody};
use crate::auth::context::ResolvedContext;
use crate::cli::duration::{LogDay, WorkDuration, hours_minutes};
use crate::cli::tasks::env::TaskEnv;
use crate::cli::time::TimeCommand;
use crate::cli::time_api::{create, entries, fetch, running_entry, save};
use crate::cli::time_clock::{day_start, parse_timestamp, shift, timestamp};
use crate::errors::KaneoError;
use crate::output;

pub async fn run(cmd: TimeCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;
    let env = TaskEnv::new(&client, ctx);

    match cmd {
        TimeCommand::Start { task, note } => start(&env, &task, note).await,
        TimeCommand::Stop { task } => stop(&env, &task).await,
        TimeCommand::Log {
            task,
            duration,
            date,
            note,
        } => log(&env, &task, duration, date, note).await,
        TimeCommand::List { task } => list(&env, &task).await,
        TimeCommand::Edit {
            id,
            duration,
            start,
            note,
        } => edit(&env, &id, duration, start, note).await,
    }
}

async fn start(env: &TaskEnv<'_>, task: &str, note: Option<String>) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(task).await?;
    if running_entry(env, &task_id).await?.is_some() {
        return Err(KaneoError::Usage(format!(
            "A timer is already running on {task}; stop it with `kaneo time stop {task}`"
        )));
    }
    let body = TimeEntryBody {
        start_time: timestamp(Utc::now()),
        end_time: None,
        description: note,
    };
    let entry = create(env, task_id, body).await?;
    output::success(&format!("Timer started on {task} ({})", entry.id));
    Ok(())
}

async fn stop(env: &TaskEnv<'_>, task: &str) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(task).await?;
    let entry = running_entry(env, &task_id)
        .await?
        .ok_or_else(|| KaneoError::Usage(format!("You have no running timer on {task}")))?;
    let body = TimeEntryBody {
        start_time: entry.start_time,
        end_time: Some(timestamp(Utc::now())),
        description: entry.description,
    };
    let saved = save(env, &entry.id, &body).await?;
    output::success(&format!("Timer stopped on {task} ({})", spent(&saved)));
    Ok(())
}

async fn log(
    env: &TaskEnv<'_>,
    task: &str,
    duration: WorkDuration,
    date: Option<LogDay>,
    note: Option<String>,
) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(task).await?;
    let (start, end) = match date {
        Some(day) => {
            let start = day_start(day)?;
            (start, shift(start, duration.delta())?)
        }
        None => {
            let end = Utc::now();
            (shift(end, -duration.delta())?, end)
        }
    };
    let body = TimeEntryBody {
        start_time: timestamp(start),
        end_time: Some(timestamp(end)),
        description: note,
    };
    let entry = create(env, task_id, body).await?;
    output::success(&format!(
        "Logged {} on {task} ({})",
        spent(&entry),
        entry.id
    ));
    Ok(())
}

async fn list(env: &TaskEnv<'_>, task: &str) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(task).await?;
    let mut entries = entries(env, &task_id).await?;
    if entries.is_empty() {
        output::warn("No time logged");
        return Ok(());
    }
    entries.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    println!(
        "{:<16} {:<8} {:<20} {:<40} ID",
        "Started", "Duration", "User", "Note"
    );
    for entry in &entries {
        let started = match DateTime::parse_from_rfc3339(&entry.start_time) {
            Ok(at) => at
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M")
                .to_string(),
            Err(_) => entry.start_time.clone(),
        };
        let user = output::truncate(entry.user_name.as_deref().unwrap_or("-"), 20);
        let note = output::truncate(entry.description.as_deref().unwrap_or(""), 40);
        println!(
            "{started:<16} {:<8} {user:<20} {note:<40} {}",
            spent(entry),
            entry.id
        );
    }

    let total: i64 = entries.iter().filter_map(TimeEntry::seconds).sum();
    let running = entries.iter().filter(|e| e.is_running()).count();
    let suffix = match running {
        0 => String::new(),
        n => format!(", {n} running"),
    };
    eprintln!(
        "\n  {} entries, {} logged{suffix}",
        entries.len(),
        hours_minutes(total)
    );
    Ok(())
}

async fn edit(
    env: &TaskEnv<'_>,
    id: &str,
    duration: Option<WorkDuration>,
    start: Option<DateTime<FixedOffset>>,
    note: Option<String>,
) -> Result<(), KaneoError> {
    if duration.is_none() && start.is_none() && note.is_none() {
        return Err(KaneoError::Usage(
            "Nothing to change: pass --duration, --start, or --note".to_owned(),
        ));
    }
    let current = fetch(env, id).await?;
    let old_start = parse_timestamp(&current.start_time)?;
    let new_start = start.unwrap_or(old_start).with_timezone(&Utc);
    let length = match (duration, current.end_time.as_deref()) {
        (Some(duration), _) => Some(duration.delta()),
        (None, Some(end)) => Some(parse_timestamp(end)?.signed_duration_since(old_start)),
        (None, None) => None,
    };
    let end_time = length
        .map(|length| shift(new_start, length))
        .transpose()?
        .map(timestamp);
    let body = TimeEntryBody {
        start_time: timestamp(new_start),
        end_time,
        description: note.or(current.description),
    };
    let saved = save(env, id, &body).await?;
    output::success(&format!("Time entry updated ({})", spent(&saved)));
    Ok(())
}

fn spent(entry: &TimeEntry) -> String {
    match (entry.is_running(), entry.seconds()) {
        (true, _) => "running".to_owned(),
        (false, Some(seconds)) => hours_minutes(seconds),
        (false, None) => "-".to_owned(),
    }
}
