use crate::api::client::ApiClient;
use crate::api::types::custom_field::{
    CreateCustomFieldBody, CustomField, FieldPosition, ReorderFieldsBody, UpdateCustomFieldBody,
};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::field_value::{encode_value, split_ids};
use crate::cli::fields::FieldCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

pub async fn run(cmd: FieldCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        FieldCommand::List { project_id, all } => {
            let project = context::require_project(project_id.as_deref(), ctx)?;
            let fields = project_fields(&client, &project).await?;
            if fields.is_empty() {
                output::warn("No custom fields found");
                return Ok(());
            }
            for f in &fields {
                print_field(f, all);
            }
        }

        FieldCommand::Create {
            name,
            field_type,
            required,
            default,
            options,
            project_id,
        } => {
            let project = context::require_project(project_id.as_deref(), ctx)?;
            let body = CreateCustomFieldBody {
                project_id: project,
                name,
                field_type,
                required,
                default_value: default.map(|d| encode_value(field_type.as_api_str(), &d)),
                options,
            };
            let field: CustomField = client
                .post("/custom-field", &body)
                .await
                .with_action("Failed to create custom field")?;
            output::success(&format!("Created field '{}' ({})", field.name, field.id));
        }

        FieldCommand::Update {
            id,
            name,
            field_type,
            required,
            default,
            clear_default,
            options,
            hide_options,
        } => {
            let encoded_default = match (default, field_type) {
                (Some(d), Some(t)) => Some(Some(encode_value(t.as_api_str(), &d))),
                (Some(d), None) => Some(Some(d)),
                (None, _) if clear_default => Some(None),
                (None, _) => None,
            };
            let body = UpdateCustomFieldBody {
                name,
                field_type,
                required,
                default_value: encoded_default,
                options: non_empty(options),
                hidden_options: non_empty(hide_options),
            };
            let field: CustomField = client
                .put(&format!("/custom-field/{id}"), &body)
                .await
                .with_action("Failed to update custom field")?;
            output::success(&format!("Updated field '{}'", field.name));
        }

        FieldCommand::Delete { id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let field: CustomField = client
                .delete(&format!("/custom-field/{id}"))
                .await
                .with_action("Failed to delete custom field")?;
            output::success(&format!("Deleted field '{}'", field.name));
        }

        FieldCommand::Reorder { project_id, order } => {
            let project = context::require_project(project_id.as_deref(), ctx)?;
            let ids = split_ids(&order);
            if ids.is_empty() {
                return Err(KaneoError::Usage(
                    "--order needs at least one field ID".to_owned(),
                ));
            }
            let body = ReorderFieldsBody {
                fields: ids
                    .into_iter()
                    .enumerate()
                    .map(|(position, id)| FieldPosition { id, position })
                    .collect(),
            };
            let _: serde_json::Value = client
                .put(&format!("/custom-field/reorder/{project}"), &body)
                .await
                .with_action("Failed to reorder custom fields")?;
            output::success("Custom fields reordered");
        }
    }

    Ok(())
}

/// Lists the custom field definitions of a project.
pub async fn project_fields(
    client: &ApiClient,
    project_id: &str,
) -> Result<Vec<CustomField>, KaneoError> {
    client
        .get(&format!("/custom-field/project/{project_id}"))
        .await
        .with_action("Failed to list custom fields")
}

fn print_field(f: &CustomField, all: bool) {
    let required = if f.required { "required" } else { "optional" };
    let options: Vec<String> = f
        .option_labels()
        .into_iter()
        .filter_map(|o| match (f.hidden_options.contains(&o), all) {
            (false, _) => Some(o),
            (true, true) => Some(format!("{o} (hidden)")),
            (true, false) => None,
        })
        .collect();
    let options = match options.is_empty() {
        true => "-".to_owned(),
        false => options.join(", "),
    };
    println!(
        "{}  {}  {}  {}  {}",
        f.name, f.field_type, required, options, f.id
    );
}

fn non_empty(items: Vec<String>) -> Option<Vec<String>> {
    match items.is_empty() {
        true => None,
        false => Some(items),
    }
}
