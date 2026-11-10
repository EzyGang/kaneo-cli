use clap::Args;
use serde::Serialize;

use crate::api::types::recurrence::{Frequency, Recurrence};
use crate::api::types::task::Task;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Weekday names indexed by the API's weekday number, where `0` is Sunday.
const WEEKDAYS: [&str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];
const MAX_INTERVAL: u32 = 99;

/// Flags shaping a repeat rule, shared by `task repeat` and `task create`.
#[derive(Args, Clone, Debug, Default, PartialEq, Eq)]
pub struct RecurrenceArgs {
    #[arg(
        long,
        help = "Repeat every N days, weeks, months, or years (1-99, default 1)"
    )]
    pub every: Option<u32>,
    #[arg(
        long,
        value_delimiter = ',',
        help = "Weekly rules only: weekdays to repeat on, such as `mon,thu`"
    )]
    pub on: Vec<String>,
    #[arg(
        long,
        help = "IANA time zone of the rule (default: this machine's zone)"
    )]
    pub time_zone: Option<String>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct RepeatArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(
        value_enum,
        help = "How often the task repeats",
        conflicts_with = "stop"
    )]
    pub frequency: Option<Frequency>,
    #[command(flatten)]
    pub rule: RecurrenceArgs,
    #[arg(
        long,
        help = "Stop the task from repeating",
        conflicts_with_all = ["every", "on", "time_zone"]
    )]
    pub stop: bool,
}

#[derive(Serialize)]
struct RecurrenceBody {
    /// `null` stops the task from repeating.
    recurrence: Option<Recurrence>,
}

impl RecurrenceArgs {
    /// Builds the repeat rule for `frequency`, or `None` when no frequency is given.
    ///
    /// Rule flags without a frequency, an interval outside 1-99, unknown weekdays, and
    /// weekdays on a non-weekly rule are rejected.
    pub fn build(&self, frequency: Option<Frequency>) -> Result<Option<Recurrence>, KaneoError> {
        let Some(frequency) = frequency else {
            if self != &Self::default() {
                return Err(KaneoError::Usage(
                    "--every, --on, and --time-zone need a repeat frequency".to_owned(),
                ));
            }
            return Ok(None);
        };
        let interval = self.every.unwrap_or(1);
        if !(1..=MAX_INTERVAL).contains(&interval) {
            return Err(KaneoError::Usage(format!(
                "--every must be between 1 and {MAX_INTERVAL}, got {interval}"
            )));
        }
        let weekdays = self.weekdays()?;
        if weekdays.is_some() && frequency != Frequency::Weekly {
            return Err(KaneoError::Usage(
                "--on only applies to weekly rules".to_owned(),
            ));
        }
        let time_zone = match &self.time_zone {
            Some(zone) => zone.clone(),
            None => local_time_zone(),
        };
        Ok(Some(Recurrence {
            frequency,
            interval,
            weekdays,
            time_zone,
        }))
    }

    /// Parses `--on` into sorted, deduplicated weekday numbers, where `0` is Sunday.
    fn weekdays(&self) -> Result<Option<Vec<u8>>, KaneoError> {
        if self.on.is_empty() {
            return Ok(None);
        }
        let mut days: Vec<u8> = self
            .on
            .iter()
            .map(|day| parse_weekday(day))
            .collect::<Result<_, _>>()?;
        days.sort_unstable();
        days.dedup();
        Ok(Some(days))
    }
}

impl RepeatArgs {
    /// Returns the rule to set, or `None` when `--stop` clears it.
    pub fn recurrence(&self) -> Result<Option<Recurrence>, KaneoError> {
        if self.stop {
            return Ok(None);
        }
        match self.frequency {
            Some(frequency) => self.rule.build(Some(frequency)),
            None => Err(KaneoError::Usage(
                "Give a frequency (daily, weekly, monthly, yearly) or --stop".to_owned(),
            )),
        }
    }
}

/// Accepts a weekday name or a prefix of at least three letters, such as `mon` or `monday`.
fn parse_weekday(input: &str) -> Result<u8, KaneoError> {
    let name = input.trim().to_lowercase();
    let index = match name.len() {
        3.. => WEEKDAYS.iter().position(|day| day.starts_with(&name)),
        _ => None,
    };
    match index.and_then(|i| u8::try_from(i).ok()) {
        Some(day) => Ok(day),
        None => Err(KaneoError::Usage(format!(
            "Unknown weekday '{input}'; use sun, mon, tue, wed, thu, fri, or sat"
        ))),
    }
}

/// Returns this machine's IANA time zone, falling back to `UTC` when it cannot be detected.
fn local_time_zone() -> String {
    match iana_time_zone::get_timezone() {
        Ok(zone) => zone,
        Err(_) => "UTC".to_owned(),
    }
}

pub async fn run(args: RepeatArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let recurrence = args.recurrence()?;
    let id = env.tasks.id(&args.task).await?;
    let body = RecurrenceBody { recurrence };
    let task: Task = env
        .client
        .put(&format!("/task/recurrence/{id}"), &body)
        .await
        .with_action("Failed to update the repeat rule")?;
    match &task.recurrence {
        Some(rule) => output::success(&format!("Task '{}' repeats {rule}", task.title)),
        None => output::success(&format!("Task '{}' no longer repeats", task.title)),
    }
    Ok(())
}
