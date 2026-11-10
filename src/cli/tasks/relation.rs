use std::fmt;

use clap::Subcommand;

use crate::api::types::relation::{CreateTaskRelationBody, RelationType, TaskRelation};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Subcommand, Clone)]
pub enum TaskRelationCommand {
    #[command(
        visible_alias = "ls",
        about = "List relations where the task is source or target"
    )]
    List {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(about = "Link two tasks")]
    Create {
        #[arg(help = "Source ticket ID or task ID")]
        source: String,
        #[arg(help = "Target ticket ID or task ID")]
        target: String,
        #[arg(value_enum, help = "Relation type")]
        relation_type: RelationType,
    },

    #[command(visible_alias = "rm", about = "Remove a relation")]
    Delete {
        #[arg(help = "Relation ID")]
        id: String,
    },
}

pub async fn run(cmd: TaskRelationCommand, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    match cmd {
        TaskRelationCommand::List { task } => {
            let task_id = env.tasks.id(&task).await?;
            let relations: Vec<TaskRelation> = env
                .client
                .get(&format!("/task-relation/{task_id}"))
                .await
                .with_action("Failed to list task relations")?;
            if relations.is_empty() {
                output::warn("No relations found for this task");
                return Ok(());
            }
            for r in &relations {
                println!(
                    "{}  {} -[{}]-> {}",
                    r.id,
                    describe(r.source_task.as_ref(), &r.source_task_id),
                    r.relation_type,
                    describe(r.target_task.as_ref(), &r.target_task_id)
                );
            }
        }

        TaskRelationCommand::Create {
            source,
            target,
            relation_type,
        } => {
            let body = CreateTaskRelationBody {
                source_task_id: env.tasks.id(&source).await?,
                target_task_id: env.tasks.id(&target).await?,
                relation_type,
            };
            let relation: TaskRelation = env
                .client
                .post("/task-relation", &body)
                .await
                .with_action("Failed to create task relation")?;
            output::success(&format!(
                "Created {} relation between {} and {} ({})",
                relation.relation_type,
                relation.source_task_id,
                relation.target_task_id,
                relation.id
            ));
        }

        TaskRelationCommand::Delete { id } => {
            let relation: TaskRelation = env
                .client
                .delete(&format!("/task-relation/{id}"))
                .await
                .with_action("Failed to delete task relation")?;
            output::success(&format!(
                "Deleted {} relation ({})",
                relation.relation_type, relation.id
            ));
        }
    }
    Ok(())
}

fn describe(summary: Option<&impl fmt::Display>, id: &str) -> String {
    match summary {
        Some(task) => format!("{task} ({id})"),
        None => id.to_owned(),
    }
}
