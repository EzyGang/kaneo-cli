use clap::Args;

use crate::api::types::priority::Priority;
use crate::api::types::recurrence::Frequency;
use crate::api::types::task::{CreateTaskBody, Task, UpdateTaskBody};
use crate::auth::context;
use crate::cli::tasks::env::TaskEnv;
use crate::cli::tasks::repeat::RecurrenceArgs;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// A command that takes one task.
#[derive(Args, Clone)]
pub struct TaskArg {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
}

#[derive(Args, Clone)]
pub struct CreateArgs {
    #[arg(long)]
    pub title: String,
    #[arg(long)]
    pub project_id: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long, value_enum, default_value_t = Priority::Low)]
    pub priority: Priority,
    #[arg(long, default_value = "planned", help = "Column slug, or `planned`")]
    pub status: String,
    #[arg(long)]
    pub due_date: Option<String>,
    #[arg(long)]
    pub start_date: Option<String>,
    #[arg(long, help = "User ID to assign, or `me`")]
    pub assignee: Option<String>,
    #[arg(long, value_enum, help = "Make the task repeat at this frequency")]
    pub repeat: Option<Frequency>,
    #[command(flatten)]
    pub recurrence: RecurrenceArgs,
}

#[derive(Args, Clone)]
pub struct UpdateArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long, value_enum)]
    pub priority: Option<Priority>,
    #[arg(long, help = "Column slug")]
    pub status: Option<String>,
    #[arg(long, help = "Due date, or `none` to clear it")]
    pub due_date: Option<String>,
    #[arg(long, help = "Start date, or `none` to clear it")]
    pub start_date: Option<String>,
    #[arg(long, help = "User ID to assign, or `me`")]
    pub assignee: Option<String>,
}

#[derive(Args, Clone)]
pub struct DeleteArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(long)]
    pub force: bool,
}

pub async fn get(args: TaskArg, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let task = env.tasks.task(&args.task).await?;
    let slug = env.project_slug(&task.project_id).await?;
    print_task(&task, &slug);
    Ok(())
}

pub async fn create(args: CreateArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let pid = context::require_project(args.project_id.as_deref(), env.ctx)?;
    let recurrence = args.recurrence.build(args.repeat)?;
    let user_id = match args.assignee.as_deref() {
        Some(user) => Some(env.user_id(user).await?),
        None => None,
    };
    let body = CreateTaskBody {
        title: args.title,
        description: args.description.unwrap_or_default(),
        priority: args.priority,
        status: args.status,
        due_date: args.due_date,
        start_date: args.start_date,
        user_id,
        recurrence,
    };
    let task: Task = env
        .client
        .post(&format!("/task/{pid}"), &body)
        .await
        .with_action("Failed to create task")?;
    let slug = env.project_slug(&task.project_id).await?;
    output::success(&format!(
        "Created task {} '{}' ({})",
        task.ticket(&slug),
        task.title,
        task.id
    ));
    if let Some(rule) = &task.recurrence {
        output::success(&format!("Repeats {rule}"));
    }
    Ok(())
}

pub async fn update(args: UpdateArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let current = env.tasks.task(&args.task).await?;
    let user_id = match args.assignee.as_deref() {
        Some(user) => Some(env.user_id(user).await?),
        None => current.user_id,
    };
    let clear_due = args.due_date.as_deref() == Some("none");
    let body = UpdateTaskBody {
        title: args.title.unwrap_or(current.title),
        description: args.description.or(current.description).unwrap_or_default(),
        priority: args
            .priority
            .map_or(current.priority, |p| p.as_api_str().to_owned()),
        status: args.status.unwrap_or(current.status),
        project_id: current.project_id,
        position: current.position.unwrap_or(0.0).round() as i64,
        due_date: date_update(args.due_date, current.due_date),
        start_date: date_update(args.start_date, current.start_date),
        user_id,
    };
    let path = format!("/task/{}", current.id);
    let task: Task = env
        .client
        .put(&path, &body)
        .await
        .with_action("Failed to update task")?;

    if clear_due {
        let _: serde_json::Value = env
            .client
            .put(
                &format!("/task/due-date/{}", task.id),
                &serde_json::json!({}),
            )
            .await
            .with_action("Failed to clear the due date")?;
    }
    output::success(&format!("Updated task '{}'", task.title));
    Ok(())
}

pub async fn delete(args: DeleteArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    if !args.force {
        output::force_required();
        return Ok(());
    }
    let id = env.tasks.id(&args.task).await?;
    let task: Task = env
        .client
        .delete(&format!("/task/{id}"))
        .await
        .with_action("Failed to delete task")?;
    output::success(&format!("Deleted task '{}'", task.title));
    Ok(())
}

/// Applies a date flag to a full-replacement body, where `none` clears the date.
fn date_update(flag: Option<String>, current: Option<String>) -> Option<String> {
    match flag.as_deref() {
        Some("none") => None,
        Some(_) => flag,
        None => current,
    }
}

fn print_task(task: &Task, slug: &str) {
    println!("{}  {}", task.ticket(slug), task.title);
    println!("  Status:    {}", task.status);
    println!("  Priority:  {}", task.priority);
    println!(
        "  Assignee:  {}",
        task.assignee_name.as_deref().unwrap_or("-")
    );
    println!("  Start:     {}", output::day(task.start_date.as_deref()));
    println!("  Due:       {}", output::day(task.due_date.as_deref()));
    if let Some(recurrence) = &task.recurrence {
        println!("  Repeats:   {recurrence}");
    }
    if let Some(counts) = task.subtask_counts.filter(|c| c.total > 0) {
        println!("  Subtasks:  {}/{}", counts.completed, counts.total);
    }
    println!("  ID:        {}", task.id);
    if let Some(desc) = task.description.as_deref().filter(|d| !d.is_empty()) {
        println!();
        println!("{desc}");
    }
}
