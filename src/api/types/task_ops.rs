use serde::{Deserialize, Serialize};

use crate::api::types::task::TaskLabel;

/// An operation of `PATCH /task/bulk`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BulkOperation {
    UpdateStatus,
    UpdatePriority,
    UpdateAssignee,
    Delete,
    AddLabel,
    RemoveLabel,
    UpdateDueDate,
}

/// The body of `PATCH /task/bulk`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkTaskBody {
    pub task_ids: Vec<String>,
    pub operation: BulkOperation,
    /// `null` clears an assignee or due date and is ignored by `delete`.
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkTaskResult {
    pub success: bool,
    pub updated_count: u64,
}

/// An open task assigned to the caller, as returned by `GET /task/assigned`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignedTask {
    pub id: String,
    pub project_id: String,
    #[serde(default)]
    pub number: Option<i64>,
    pub title: String,
    /// The slug of the column the task sits in.
    pub status: String,
    #[serde(default)]
    pub status_name: Option<String>,
    pub priority: String,
    #[serde(default)]
    pub due_date: Option<String>,
    pub project_name: String,
    pub project_slug: String,
    #[serde(default)]
    pub labels: Vec<TaskLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AssignedTasks {
    pub tasks: Vec<AssignedTask>,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TaskImportResult {
    pub results: TaskImportSummary,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TaskImportSummary {
    pub total: u64,
    pub successful: u64,
    pub failed: u64,
    /// Per-task outcomes; their shape is unspecified beyond a `success` flag.
    #[serde(default)]
    pub tasks: Vec<serde_json::Value>,
}

impl AssignedTask {
    /// Returns the ticket ID, such as `KAN-12`, or `-` when the task has no number.
    #[must_use]
    pub fn ticket(&self) -> String {
        match self.number {
            Some(number) => format!("{}-{number}", self.project_slug),
            None => "-".to_owned(),
        }
    }
}
