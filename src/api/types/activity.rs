use serde::Deserialize;
use serde_json::Value;

/// A task activity returned by `GET /activity/{taskId}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    pub task_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub created_at: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub event_data: Option<Value>,
    #[serde(default)]
    pub external_user_name: Option<String>,
}

/// A workspace-wide activity returned by `GET /activity/workspace/{workspaceId}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceActivity {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub created_at: String,
    #[serde(default)]
    pub event_data: Option<Value>,
    #[serde(default)]
    pub excerpt: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub external_user_name: Option<String>,
    pub task_id: String,
    pub task_title: String,
    #[serde(default)]
    pub task_number: Option<i64>,
    pub project_slug: String,
}

impl Activity {
    /// Returns who caused the activity: the imported author, the user ID, or `system`.
    #[must_use]
    pub fn actor(&self) -> &str {
        self.external_user_name
            .as_deref()
            .or(self.user_id.as_deref())
            .unwrap_or("system")
    }

    #[must_use]
    pub fn summary(&self) -> String {
        event_summary(self.content.as_deref(), self.event_data.as_ref())
    }
}

impl WorkspaceActivity {
    /// Returns who caused the activity: the user name, the imported author, or `system`.
    #[must_use]
    pub fn actor(&self) -> &str {
        self.user_name
            .as_deref()
            .or(self.external_user_name.as_deref())
            .or(self.user_id.as_deref())
            .unwrap_or("system")
    }

    /// Returns the ticket ID of the task, such as `KAN-12`, or the task ID when it has no number.
    #[must_use]
    pub fn ticket(&self) -> String {
        match self.task_number {
            Some(number) => format!("{}-{number}", self.project_slug),
            None => self.task_id.clone(),
        }
    }

    #[must_use]
    pub fn summary(&self) -> String {
        event_summary(self.excerpt.as_deref(), self.event_data.as_ref())
    }
}

/// Describes an event by its free text, or else by the scalar fields of its event data as
/// `key: value` pairs.
#[must_use]
pub fn event_summary(text: Option<&str>, event_data: Option<&Value>) -> String {
    if let Some(text) = text.map(str::trim).filter(|t| !t.is_empty()) {
        return text.replace('\n', " ");
    }
    let Some(Value::Object(fields)) = event_data else {
        return String::new();
    };
    fields
        .iter()
        .filter_map(|(key, value)| {
            let rendered = match value {
                Value::String(s) if !s.is_empty() => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => return None,
            };
            Some(format!("{key}: {rendered}"))
        })
        .collect::<Vec<String>>()
        .join(", ")
}
