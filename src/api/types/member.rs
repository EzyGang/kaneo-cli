use serde::Deserialize;

/// A workspace member returned by `GET /workspace/{id}/members`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WorkspaceMember {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
    /// Comma-separated workspace roles, such as `owner,viewer`.
    pub role: String,
}
