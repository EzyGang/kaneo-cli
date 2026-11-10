use serde::{Deserialize, Serialize};

/// A time entry; `user_name` is only filled in by `GET /time-entry/task/{taskId}`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub start_time: String,
    /// Absent while the timer is still running.
    #[serde(default)]
    pub end_time: Option<String>,
    /// Elapsed seconds, set once the entry has an end time.
    #[serde(default)]
    pub duration: Option<f64>,
}

/// Body of `PUT /time-entry/{id}`, which replaces every field.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntryBody {
    pub start_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeEntryBody {
    pub task_id: String,
    #[serde(flatten)]
    pub entry: TimeEntryBody,
}

impl TimeEntry {
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.end_time.is_none()
    }

    /// Returns the elapsed whole seconds of a finished entry.
    #[must_use]
    pub fn seconds(&self) -> Option<i64> {
        self.duration.map(|seconds| seconds.round() as i64)
    }
}
