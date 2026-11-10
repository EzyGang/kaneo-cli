use clap::Args;
use serde::Serialize;

use crate::api::types::priority::Priority;
use crate::api::types::task::{MoveTaskResponse, Task};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone)]
pub struct StatusArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(help = "Column slug, such as `in-progress`")]
    pub status: String,
}

#[derive(Args, Clone)]
pub struct PriorityArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(value_enum)]
    pub priority: Priority,
}

#[derive(Args, Clone)]
pub struct AssignArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(long, help = "User ID, or `me`; omit to unassign")]
    pub user_id: Option<String>,
}

#[derive(Args, Clone)]
pub struct MoveArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(help = "Destination project ID")]
    pub project_id: String,
    #[arg(long, help = "Destination column slug")]
    pub status: Option<String>,
}

#[derive(Serialize)]
struct StatusBody {
    status: String,
}

#[derive(Serialize)]
struct PriorityBody {
    priority: Priority,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssignBody {
    /// `null` unassigns the task.
    user_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MoveBody {
    destination_project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_status: Option<String>,
}

pub async fn status(args: StatusArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let id = env.tasks.id(&args.task).await?;
    let body = StatusBody {
        status: args.status,
    };
    let task: Task = env
        .client
        .put(&format!("/task/status/{id}"), &body)
        .await
        .with_action("Failed to change status")?;
    output::success(&format!("Task '{}' status -> {}", task.title, task.status));
    Ok(())
}

pub async fn priority(args: PriorityArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let id = env.tasks.id(&args.task).await?;
    let body = PriorityBody {
        priority: args.priority,
    };
    let task: Task = env
        .client
        .put(&format!("/task/priority/{id}"), &body)
        .await
        .with_action("Failed to change priority")?;
    output::success(&format!(
        "Task '{}' priority -> {}",
        task.title, task.priority
    ));
    Ok(())
}

pub async fn assign(args: AssignArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let id = env.tasks.id(&args.task).await?;
    let user_id = match args.user_id.as_deref() {
        Some(user) => Some(env.user_id(user).await?),
        None => None,
    };
    let task: Task = env
        .client
        .put(&format!("/task/assignee/{id}"), &AssignBody { user_id })
        .await
        .with_action("Failed to assign task")?;
    match task.user_id {
        Some(user) => output::success(&format!("Task '{}' assigned to {user}", task.title)),
        None => output::success(&format!("Task '{}' unassigned", task.title)),
    }
    Ok(())
}

pub async fn move_task(args: MoveArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let id = env.tasks.id(&args.task).await?;
    let body = MoveBody {
        destination_project_id: args.project_id,
        destination_status: args.status,
    };
    let result: MoveTaskResponse = env
        .client
        .put(&format!("/task/move/{id}"), &body)
        .await
        .with_action("Failed to move task")?;
    output::success(&format!(
        "Task '{}' moved to project {} ({})",
        result.task.title, result.destination_project_id, result.task.status
    ));
    Ok(())
}
