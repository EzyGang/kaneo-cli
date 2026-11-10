use serde::Deserialize;
use serde_json::Value;

use crate::api::types::activity::event_summary;

/// A notification returned by `GET /notification` and the read/unread mutations.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub event_data: Option<Value>,
    #[serde(default)]
    pub is_read: Option<bool>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    pub created_at: String,
}

/// The `{ success }` body returned by bulk notification endpoints and `DELETE /notification/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct NotificationBulkResult {
    pub success: bool,
}

impl Notification {
    #[must_use]
    pub fn read(&self) -> bool {
        self.is_read.unwrap_or(false)
    }

    /// Returns the best human-readable text: title and content, else the event type and its data.
    #[must_use]
    pub fn text(&self) -> String {
        let title = self.title.as_deref().filter(|t| !t.is_empty());
        let content = self.content.as_deref().filter(|c| !c.is_empty());
        match (title, content) {
            (Some(title), Some(content)) => format!("{title}: {content}"),
            (Some(text), None) | (None, Some(text)) => text.to_owned(),
            (None, None) => {
                let kind = self.kind.replace('_', " ");
                match event_summary(None, self.event_data.as_ref()) {
                    details if details.is_empty() => kind,
                    details => format!("{kind} ({details})"),
                }
            }
        }
    }
}
