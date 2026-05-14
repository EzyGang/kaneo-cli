use crate::api::client::ApiClient;
use crate::api::types::{Column, CreateColumnBody};
use crate::auth::context;
use crate::cli::columns::ColumnCommand;
use crate::output;

pub async fn run(
    cmd: ColumnCommand,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

    match cmd {
        ColumnCommand::List { project_id } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let columns: Vec<Column> = client
                .get(&format!("/column/{pid}"))
                .await
                .map_err(|e| crate::errors::api_error("failed to list columns".to_owned(), e))?;

            if columns.is_empty() {
                output::warn("No columns found");
                return Ok(());
            }
            for c in &columns {
                let fin = if c.is_final.unwrap_or(false) {
                    " [final]"
                } else {
                    ""
                };
                println!("{}  {}{fin}", c.name, c.id);
            }
        }

        ColumnCommand::Create {
            name,
            project_id,
            icon,
            color,
            is_final,
        } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let body = CreateColumnBody {
                name,
                icon,
                color,
                is_final: if is_final { Some(true) } else { None },
            };
            let col: Column = client
                .post(&format!("/column/{pid}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to create column".to_owned(), e))?;

            output::success(&format!("Created column '{}' ({})", col.name, col.id));
        }

        ColumnCommand::Update {
            id,
            name,
            icon,
            color,
            is_final,
        } => {
            #[derive(serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            struct UpdateBody {
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                icon: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                color: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                is_final: Option<bool>,
            }
            let body = UpdateBody {
                name,
                icon,
                color,
                is_final,
            };
            let col: Column = client
                .put(&format!("/column/{id}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to update column".to_owned(), e))?;

            output::success(&format!("Updated column '{}'", col.name));
        }

        ColumnCommand::Delete { id, force } => {
            if !force {
                eprintln!(
                    "  {} Re-run with `--force` to confirm deletion",
                    output::dim("note:")
                );
                return Ok(());
            }
            let col: Column = client
                .delete(&format!("/column/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Column", &id, e))?;

            output::success(&format!("Deleted column '{}'", col.name));
        }

        ColumnCommand::Reorder { project_id, order } => {
            #[derive(serde::Serialize)]
            struct ReorderBody {
                columns: Vec<ColumnPos>,
            }
            #[derive(serde::Serialize)]
            struct ColumnPos {
                id: String,
                position: i64,
            }

            let columns: Vec<ColumnPos> = order
                .iter()
                .enumerate()
                .map(|(i, id)| ColumnPos {
                    id: id.clone(),
                    position: i as i64,
                })
                .collect();

            let _result: serde_json::Value = client
                .put(
                    &format!("/column/reorder/{project_id}"),
                    &ReorderBody { columns },
                )
                .await
                .map_err(|e| crate::errors::api_error("failed to reorder columns".to_owned(), e))?;

            output::success("Columns reordered");
        }
    }

    Ok(())
}
