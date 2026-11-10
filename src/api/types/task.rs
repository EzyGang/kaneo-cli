use serde::{Deserialize, Serialize};

use crate::api::types::priority::Priority;
use crate::api::types::recurrence::Recurrence;

/// A task as returned by the task, board, and mutation endpoints.
///
/// Board rows omit `columnId`, `workspaceId`, and custom fields; detail rows omit labels.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    #[serde(default)]
    pub position: Option<f64>,
    #[serde(default)]
    pub number: Option<i64>,
    #[serde(default)]
    pub user_id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    /// The description was too large to embed and must be fetched separately.
    #[serde(default)]
    pub description_deferred: bool,
    pub status: String,
    pub priority: String,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub recurrence: Option<Recurrence>,
    pub created_at: String,
    #[serde(default)]
    pub workspace_id: Option<String>,
    #[serde(default)]
    pub column_id: Option<String>,
    #[serde(default)]
    pub assignee_name: Option<String>,
    #[serde(default)]
    pub assignee_id: Option<String>,
    #[serde(default)]
    pub subtask_counts: Option<SubtaskCounts>,
    #[serde(default)]
    pub labels: Vec<TaskLabel>,
    #[serde(default)]
    pub custom_fields: Vec<CustomFieldEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct SubtaskCounts {
    pub completed: u32,
    pub total: u32,
}

/// A label summary embedded in a board task.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TaskLabel {
    pub id: String,
    pub name: String,
    pub color: String,
}

/// A custom field value embedded in a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldEntry {
    pub field_id: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBody {
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}

/// The full replacement body of `PUT /task/{id}`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskBody {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub project_id: String,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveTaskResponse {
    pub task: Task,
    pub source_project_id: String,
    pub destination_project_id: String,
}

impl Task {
    /// Returns the ticket ID, such as `KAN-12`, when the project slug and number are known.
    #[must_use]
    pub fn ticket(&self, project_slug: &str) -> String {
        match self.number {
            Some(number) => format!("{project_slug}-{number}"),
            None => "-".to_owned(),
        }
    }
}
