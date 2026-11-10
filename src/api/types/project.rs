use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub workspace_id: String,
    pub slug: String,
    pub icon: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub is_public: Option<bool>,
    #[serde(default)]
    pub archived_at: Option<String>,
    #[serde(default)]
    pub position: Option<f64>,
    #[serde(default)]
    pub last_task_number: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectBody {
    pub name: String,
    pub workspace_id: String,
    pub slug: String,
    pub icon: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectBody {
    pub name: String,
    pub icon: String,
    pub slug: String,
    pub description: String,
    pub is_public: bool,
}

/// The response of `PUT /project/{id}/move`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovedProject {
    #[serde(flatten)]
    pub project: Project,
    /// Number of tasks the move left without an assignee.
    pub unassigned_task_count: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveProjectBody {
    pub workspace_id: String,
}

/// The body of `PUT /project/reorder`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReorderProjectsBody {
    pub projects: Vec<ProjectPosition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectPosition {
    pub id: String,
    pub position: usize,
}

impl UpdateProjectBody {
    /// Builds a full replacement body that keeps every field of `project` unless overridden.
    #[must_use]
    pub fn from_project(project: Project) -> Self {
        Self {
            name: project.name,
            icon: project.icon.unwrap_or_default(),
            slug: project.slug,
            description: project.description.unwrap_or_default(),
            is_public: project.is_public.unwrap_or(false),
        }
    }
}
