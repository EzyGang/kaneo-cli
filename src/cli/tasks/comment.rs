use clap::Subcommand;

use crate::api::types::comment::{Comment, CommentBody};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Subcommand, Clone)]
pub enum TaskCommentCommand {
    #[command(visible_alias = "ls", about = "List comments on a task, oldest first")]
    List {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(about = "Add a comment to a task")]
    Add {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "Comment text")]
        content: String,
    },

    #[command(about = "Edit one of your comments")]
    Update {
        #[arg(help = "Comment ID")]
        id: String,
        #[arg(help = "Updated comment text")]
        content: String,
    },

    #[command(visible_alias = "rm", about = "Delete one of your comments")]
    Delete {
        #[arg(help = "Comment ID")]
        id: String,
    },
}

pub async fn run(cmd: TaskCommentCommand, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    match cmd {
        TaskCommentCommand::List { task } => {
            let task_id = env.tasks.id(&task).await?;
            let comments: Vec<Comment> = env
                .client
                .get(&format!("/comment/{task_id}"))
                .await
                .with_action("Failed to list comments")?;
            if comments.is_empty() {
                output::warn("No comments");
                return Ok(());
            }
            for c in &comments {
                let author = c.user.as_ref().map_or("unknown", |u| u.name.as_str());
                let content = c.content.as_deref().unwrap_or("");
                println!("{}  {}  {}", c.id, author, content);
            }
        }

        TaskCommentCommand::Add { task, content } => {
            let task_id = env.tasks.id(&task).await?;
            let comment: Comment = env
                .client
                .post(&format!("/comment/{task_id}"), &CommentBody { content })
                .await
                .with_action("Failed to add comment")?;
            output::success(&format!("Comment added ({})", comment.id));
        }

        TaskCommentCommand::Update { id, content } => {
            let _: Comment = env
                .client
                .put(&format!("/comment/{id}"), &CommentBody { content })
                .await
                .with_action("Failed to update comment")?;
            output::success("Comment updated");
        }

        TaskCommentCommand::Delete { id } => {
            let _: Comment = env
                .client
                .delete(&format!("/comment/{id}"))
                .await
                .with_action("Failed to delete comment")?;
            output::success("Comment deleted");
        }
    }
    Ok(())
}
