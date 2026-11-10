use clap::Subcommand;
use serde::Serialize;

use crate::api::types::label::Label;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Subcommand, Clone)]
pub enum TaskLabelCommand {
    #[command(visible_alias = "ls", about = "List labels on a task")]
    List {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(about = "Attach a workspace label to a task")]
    Attach {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "Workspace label ID (see `kaneo label list`)")]
        label_id: String,
    },

    #[command(about = "Detach a label from a task")]
    Detach {
        #[arg(help = "Task label ID (see `kaneo task label list`)")]
        label_id: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AttachBody {
    task_id: String,
}

pub async fn run(cmd: TaskLabelCommand, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    match cmd {
        TaskLabelCommand::List { task } => {
            let task_id = env.tasks.id(&task).await?;
            let labels: Vec<Label> = env
                .client
                .get(&format!("/label/task/{task_id}"))
                .await
                .with_action("Failed to list task labels")?;
            if labels.is_empty() {
                output::warn("No labels on task");
                return Ok(());
            }
            for l in &labels {
                println!("{}  {}  {}", l.name, l.color, l.id);
            }
        }

        TaskLabelCommand::Attach { task, label_id } => {
            let task_id = env.tasks.id(&task).await?;
            let label: Label = env
                .client
                .put(&format!("/label/{label_id}/task"), &AttachBody { task_id })
                .await
                .with_action("Failed to attach label")?;
            output::success(&format!("Label '{}' attached to task", label.name));
        }

        TaskLabelCommand::Detach { label_id } => {
            let label: Label = env
                .client
                .delete(&format!("/label/{label_id}/task"))
                .await
                .with_action("Failed to detach label")?;
            output::success(&format!("Label '{}' detached from task", label.name));
        }
    }
    Ok(())
}
