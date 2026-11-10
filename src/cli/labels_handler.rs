use std::time::Duration;

use crate::api::client::ApiClient;
use crate::api::types::label::{CreateLabelBody, Label, UpdateLabelBody};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::labels::LabelCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Pause between `DELETE` repeats while a large label cascade is still running.
const DELETE_POLL_INTERVAL: Duration = Duration::from_secs(1);

pub async fn run(cmd: LabelCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        LabelCommand::List { workspace_id } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let labels: Vec<Label> = client
                .get(&format!("/label/workspace/{ws}"))
                .await
                .with_action("Failed to list labels")?;
            if labels.is_empty() {
                output::warn("No labels found");
                return Ok(());
            }
            for l in &labels {
                println!("{}  {}  {}", l.name, l.color, l.id);
            }
        }

        LabelCommand::Get { id } => {
            let label = fetch(&client, &id).await?;
            println!("{}", label.name);
            println!("  Color:     {}", label.color);
            println!("  ID:        {}", label.id);
            if let Some(ws) = &label.workspace_id {
                println!("  Workspace: {ws}");
            }
        }

        LabelCommand::Create {
            name,
            workspace_id,
            color,
        } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let body = CreateLabelBody {
                name,
                color,
                workspace_id: ws,
                task_id: None,
            };
            let label: Label = client
                .post("/label", &body)
                .await
                .with_action("Failed to create label")?;
            output::success(&format!("Created label '{}' ({})", label.name, label.id));
        }

        LabelCommand::Update { id, name, color } => {
            let current = fetch(&client, &id).await?;
            let body = UpdateLabelBody {
                name: name.unwrap_or(current.name),
                color: color.unwrap_or(current.color),
            };
            let label: Label = client
                .put(&format!("/label/{id}"), &body)
                .await
                .with_action("Failed to update label")?;
            output::success(&format!("Updated label '{}'", label.name));
        }

        LabelCommand::Delete { id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let label = delete_until_done(&client, &id).await?;
            output::success(&format!("Deleted label '{}'", label.name));
        }
    }

    Ok(())
}

async fn fetch(client: &ApiClient, id: &str) -> Result<Label, KaneoError> {
    client
        .get(&format!("/label/{id}"))
        .await
        .with_action("Failed to get label")
}

/// Repeats `DELETE /label/{id}` while the server reports the cascade as pending.
async fn delete_until_done(client: &ApiClient, id: &str) -> Result<Label, KaneoError> {
    let path = format!("/label/{id}");
    loop {
        let label: Label = client
            .delete(&path)
            .await
            .with_action("Failed to delete label")?;
        if !label.pending_deletion {
            return Ok(label);
        }
        output::warn(&format!(
            "Removing label '{}' from tasks, continuing…",
            label.name
        ));
        tokio::time::sleep(DELETE_POLL_INTERVAL).await;
    }
}
