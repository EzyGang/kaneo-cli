use crate::api::client::ApiClient;
use crate::api::types::{CreateLabelBody, Label};
use crate::auth::context;
use crate::cli::labels::LabelCommand;
use crate::output;

pub async fn run(
    cmd: LabelCommand,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

    match cmd {
        LabelCommand::List { workspace_id } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let labels: Vec<Label> = client
                .get(&format!("/label/workspace/{ws}"))
                .await
                .map_err(|e| crate::errors::api_error("failed to list labels".to_owned(), e))?;

            if labels.is_empty() {
                output::warn("No labels found");
                return Ok(());
            }
            for l in &labels {
                println!("{}  {}  {}", l.name, l.color, l.id);
            }
        }

        LabelCommand::Get { id } => {
            let label: Label = client
                .get(&format!("/label/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Label", &id, e))?;

            println!("{}", label.name);
            println!("  Color:  {}", label.color);
            println!("  ID:     {}", label.id);
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
                color: color.unwrap_or_else(|| "#6366f1".to_owned()),
                workspace_id: ws,
                task_id: None,
            };
            let label: Label = client
                .post("/label", &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to create label".to_owned(), e))?;

            output::success(&format!("Created label '{}' ({})", label.name, label.id));
        }

        LabelCommand::Update { id, name, color } => {
            #[derive(serde::Serialize)]
            struct UpdateBody {
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                color: Option<String>,
            }
            let body = UpdateBody { name, color };
            let label: Label = client
                .put(&format!("/label/{id}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to update label".to_owned(), e))?;

            output::success(&format!("Updated label '{}'", label.name));
        }

        LabelCommand::Delete { id, force } => {
            if !force {
                eprintln!(
                    "  {} Re-run with `--force` to confirm deletion",
                    output::dim("note:")
                );
                return Ok(());
            }
            let label: Label = client
                .delete(&format!("/label/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Label", &id, e))?;

            output::success(&format!("Deleted label '{}'", label.name));
        }
    }

    Ok(())
}
