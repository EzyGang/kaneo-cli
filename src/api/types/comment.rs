use serde::{Deserialize, Serialize};

/// A comment from `GET /comment/{taskId}`, or the activity row returned by comment mutations.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub task_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub user: Option<CommentUser>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CommentUser {
    pub name: String,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommentBody {
    pub content: String,
}
