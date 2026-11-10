use crate::api::client::ApiClient;
use crate::api::types::column::Column;
use crate::api::types::workflow::{SetWorkflowRuleBody, WorkflowRule};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::workflows::WorkflowCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

pub async fn run(cmd: WorkflowCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        WorkflowCommand::List { project_id } => {
            let project = context::require_project(project_id.as_deref(), ctx)?;
            let rules: Vec<WorkflowRule> = client
                .get(&format!("/workflow-rule/{project}"))
                .await
                .with_action("Failed to list workflow rules")?;
            if rules.is_empty() {
                output::warn("No workflow rules found");
                return Ok(());
            }
            for r in &rules {
                let column = r
                    .column_slug
                    .as_deref()
                    .or(r.column_name.as_deref())
                    .or(r.column_id.as_deref())
                    .unwrap_or("-");
                println!(
                    "{}  {}  {}  {}",
                    r.integration_type, r.event_type, column, r.id
                );
            }
        }

        WorkflowCommand::Set {
            integration,
            event,
            column,
            project_id,
        } => {
            let project = context::require_project(project_id.as_deref(), ctx)?;
            let column_id = resolve_column(&client, &project, &column).await?;
            let body = SetWorkflowRuleBody {
                integration_type: integration,
                event_type: event,
                column_id,
            };
            let rule: WorkflowRule = client
                .put(&format!("/workflow-rule/{project}"), &body)
                .await
                .with_action("Failed to set workflow rule")?;
            output::success(&format!("Workflow rule saved ({})", rule.id));
        }

        WorkflowCommand::Delete { id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let _: WorkflowRule = client
                .delete(&format!("/workflow-rule/{id}"))
                .await
                .with_action("Failed to delete workflow rule")?;
            output::success("Workflow rule deleted");
        }
    }

    Ok(())
}

/// Resolves a column ID or slug to the column's ID within a project.
async fn resolve_column(
    client: &ApiClient,
    project_id: &str,
    column: &str,
) -> Result<String, KaneoError> {
    let columns: Vec<Column> = client
        .get(&format!("/column/{project_id}"))
        .await
        .with_action("Failed to list columns")?;
    columns
        .into_iter()
        .find(|c| c.id == column || c.slug == column)
        .map(|c| c.id)
        .ok_or_else(|| KaneoError::Usage(format!("No column '{column}' in this project")))
}
