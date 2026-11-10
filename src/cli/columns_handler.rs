use crate::api::types::column::{
    Column, ColumnPosition, CreateColumnBody, ReorderColumnsBody, UpdateColumnBody,
};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::columns::ColumnCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

pub async fn run(cmd: ColumnCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        ColumnCommand::List { project_id } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let columns: Vec<Column> = client
                .get(&format!("/column/{pid}"))
                .await
                .with_action("Failed to list columns")?;
            print_columns(&columns);
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
                is_final: is_final.then_some(true),
            };
            let col: Column = client
                .post(&format!("/column/{pid}"), &body)
                .await
                .with_action("Failed to create column")?;
            output::success(&format!(
                "Created column '{}' ({}) ({})",
                col.name, col.slug, col.id
            ));
        }

        ColumnCommand::Update {
            id,
            name,
            icon,
            color,
            is_final,
        } => {
            let body = UpdateColumnBody {
                name,
                icon,
                color,
                is_final,
            };
            let col: Column = client
                .put(&format!("/column/{id}"), &body)
                .await
                .with_action("Failed to update column")?;
            output::success(&format!("Updated column '{}'", col.name));
        }

        ColumnCommand::Delete { id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let col: Column = client
                .delete(&format!("/column/{id}"))
                .await
                .with_action("Failed to delete column (it must be empty)")?;
            output::success(&format!("Deleted column '{}'", col.name));
        }

        ColumnCommand::Reorder { project_id, order } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let columns = order
                .into_iter()
                .enumerate()
                .map(|(position, id)| ColumnPosition { id, position })
                .collect();
            let reordered: Vec<Column> = client
                .put(
                    &format!("/column/reorder/{pid}"),
                    &ReorderColumnsBody { columns },
                )
                .await
                .with_action("Failed to reorder columns")?;
            output::success("Columns reordered");
            print_columns(&reordered);
        }
    }

    Ok(())
}

fn print_columns(columns: &[Column]) {
    if columns.is_empty() {
        output::warn("No columns found");
        return;
    }
    for c in columns {
        let fin = if c.is_final { " [final]" } else { "" };
        println!("{}  ({})  {}{fin}", c.name, c.slug, c.id);
    }
}
