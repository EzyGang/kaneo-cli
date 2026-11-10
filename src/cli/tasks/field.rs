use clap::Args;

use crate::api::types::custom_field::{CustomFieldValue, SetFieldValueBody};
use crate::cli::field_value::{encode_value, resolve_field};
use crate::cli::fields_handler::project_fields;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone)]
pub struct FieldArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,
    #[arg(
        help = "Custom field ID or name (case-insensitive)",
        requires = "value"
    )]
    pub field: Option<String>,
    #[arg(help = "New value (comma-separated for multiselect)")]
    pub value: Option<String>,
}

/// Shows a task's custom field values, or sets one when a field and value are given.
pub async fn run(args: FieldArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let (field_arg, raw) = match (args.field, args.value) {
        (Some(f), Some(v)) => (f, v),
        _ => return show(&args.task, env).await,
    };

    let task = env.tasks.task(&args.task).await?;
    let fields = project_fields(env.client, &task.project_id).await?;
    let field = resolve_field(&fields, &field_arg)?;
    let body = SetFieldValueBody {
        task_id: task.id.clone(),
        field_id: field.id.clone(),
        value: encode_value(&field.field_type, &raw),
    };
    let _: serde_json::Value = env
        .client
        .put("/custom-field/value", &body)
        .await
        .with_action("Failed to set custom field value")?;
    output::success(&format!("Set '{}' on task", field.name));
    Ok(())
}

async fn show(task: &str, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(task).await?;
    let mut values: Vec<CustomFieldValue> = env
        .client
        .get(&format!("/custom-field/task/{task_id}"))
        .await
        .with_action("Failed to get custom field values")?;
    if values.is_empty() {
        output::warn("No custom field values");
        return Ok(());
    }
    values.sort_by(|a, b| a.field_position.total_cmp(&b.field_position));
    for v in &values {
        let value = v.value.as_deref().unwrap_or("-");
        println!(
            "{}  {}  {}  {}",
            v.field_name, v.field_type, value, v.field_id
        );
    }
    Ok(())
}
