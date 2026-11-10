use serde::Deserialize;

/// A workspace returned by `GET /workspace` and `GET /workspace/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    /// The caller's role, or `None` for instance admins who are not members.
    #[serde(default)]
    pub role: Option<String>,
}
