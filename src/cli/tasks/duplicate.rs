use clap::Args;
use serde::Serialize;

use crate::api::types::task::Task;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct DuplicateArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(long, help = "Title of the copy (default: the original title)")]
    pub title: Option<String>,
}

#[derive(Serialize)]
struct DuplicateBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

pub async fn run(args: DuplicateArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let id = env.tasks.id(&args.task).await?;
    let body = DuplicateBody { title: args.title };
    let task: Task = env
        .client
        .post(&format!("/task/duplicate/{id}"), &body)
        .await
        .with_action("Failed to duplicate task")?;
    let slug = env.project_slug(&task.project_id).await?;
    output::success(&format!(
        "Duplicated as {} '{}' ({})",
        task.ticket(&slug),
        task.title,
        task.id
    ));
    Ok(())
}
