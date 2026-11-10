use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub workspace_id: Option<String>,
    /// Set on `202` responses of `DELETE /label/{id}` while the cascade is still running.
    #[serde(default)]
    pub pending_deletion: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabelBody {
    pub name: String,
    pub color: String,
    pub workspace_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateLabelBody {
    pub name: String,
    pub color: String,
}
