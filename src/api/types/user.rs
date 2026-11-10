use serde::Deserialize;

/// The authenticated user returned by `GET /user/me`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
    /// The instance-wide role, such as `admin`, or `None` for regular users.
    #[serde(default)]
    pub role: Option<String>,
}
