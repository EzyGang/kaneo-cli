use clap::{ArgGroup, Args};

use crate::api::types::priority::Priority;
use crate::api::types::task_ops::{BulkOperation, BulkTaskBody, BulkTaskResult};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone, Debug, Default, PartialEq, Eq)]
#[command(group(ArgGroup::new("operation").required(true).multiple(false)))]
pub struct BulkArgs {
    #[arg(required = true, help = "Ticket IDs (e.g. KAN-12) or task IDs")]
    pub tasks: Vec<String>,
    #[arg(long, group = "operation", help = "Move the tasks to this column slug")]
    pub status: Option<String>,
    #[arg(
        long,
        value_enum,
        group = "operation",
        help = "Set the tasks' priority"
    )]
    pub priority: Option<Priority>,
    #[arg(
        long,
        group = "operation",
        help = "Assign the tasks to this user ID, or `me`"
    )]
    pub assignee: Option<String>,
    #[arg(long, group = "operation", help = "Unassign the tasks")]
    pub unassign: bool,
    #[arg(
        long,
        group = "operation",
        help = "Set the due date, or `none` to clear it"
    )]
    pub due: Option<String>,
    #[arg(long, group = "operation", help = "Add this workspace label ID")]
    pub add_label: Option<String>,
    #[arg(long, group = "operation", help = "Remove this label ID")]
    pub remove_label: Option<String>,
    #[arg(
        long,
        group = "operation",
        help = "Delete the tasks (requires --force)"
    )]
    pub delete: bool,
    #[arg(long, help = "Confirm --delete")]
    pub force: bool,
}

/// The operation and value sent to `PATCH /task/bulk`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkAction {
    pub operation: BulkOperation,
    /// `None` clears an assignee or due date; an assignee may still be `me`.
    pub value: Option<String>,
}

impl BulkArgs {
    /// Maps the chosen operation flag to the API operation and its value.
    pub fn action(&self) -> Result<BulkAction, KaneoError> {
        let (operation, value) = if let Some(status) = &self.status {
            (BulkOperation::UpdateStatus, Some(status.clone()))
        } else if let Some(priority) = self.priority {
            let value = priority.as_api_str().to_owned();
            (BulkOperation::UpdatePriority, Some(value))
        } else if let Some(user) = &self.assignee {
            (BulkOperation::UpdateAssignee, Some(user.clone()))
        } else if self.unassign {
            (BulkOperation::UpdateAssignee, None)
        } else if let Some(due) = &self.due {
            let value = (due != "none").then(|| due.clone());
            (BulkOperation::UpdateDueDate, value)
        } else if let Some(label) = &self.add_label {
            (BulkOperation::AddLabel, Some(label.clone()))
        } else if let Some(label) = &self.remove_label {
            (BulkOperation::RemoveLabel, Some(label.clone()))
        } else if self.delete {
            (BulkOperation::Delete, None)
        } else {
            return Err(KaneoError::Usage(
                "Choose one operation, such as --status or --delete".to_owned(),
            ));
        };
        Ok(BulkAction { operation, value })
    }
}

pub async fn run(args: BulkArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let action = args.action()?;
    if action.operation == BulkOperation::Delete && !args.force {
        output::force_required();
        return Ok(());
    }
    let value = match (action.operation, action.value) {
        (BulkOperation::UpdateAssignee, Some(user)) => Some(env.user_id(&user).await?),
        (_, value) => value,
    };
    let mut task_ids: Vec<String> = Vec::with_capacity(args.tasks.len());
    for task in &args.tasks {
        task_ids.push(env.tasks.id(task).await?);
    }
    let body = BulkTaskBody {
        task_ids,
        operation: action.operation,
        value,
    };
    let result: BulkTaskResult = env
        .client
        .patch("/task/bulk", &body)
        .await
        .with_action("Failed to update tasks")?;
    match result.success {
        true => output::success(&format!(
            "Updated {} of {} tasks",
            result.updated_count,
            body.task_ids.len()
        )),
        false => output::warn(&format!(
            "Bulk update did not fully succeed: {} of {} tasks updated",
            result.updated_count,
            body.task_ids.len()
        )),
    }
    Ok(())
}
