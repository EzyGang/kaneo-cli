use std::collections::HashSet;

use serde::Serialize;

use crate::api::client::ApiClient;
use crate::api::types::project::{
    MoveProjectBody, MovedProject, Project, ProjectPosition, ReorderProjectsBody,
};
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceQuery<'a> {
    workspace_id: &'a str,
}

/// Moves a project with its tasks into another workspace, once confirmed with `--force`.
pub async fn move_project(
    client: &ApiClient,
    id: &str,
    workspace_id: String,
    force: bool,
) -> Result<(), KaneoError> {
    if !force {
        eprintln!(
            "  {} Moving a project transfers all its tasks; relations to tasks in other projects must be removed first",
            output::dim("note:")
        );
        eprintln!(
            "  {} Re-run with `--force` to confirm the move",
            output::dim("note:")
        );
        return Ok(());
    }
    let moved: MovedProject = client
        .put(
            &format!("/project/{id}/move"),
            &MoveProjectBody { workspace_id },
        )
        .await
        .with_action("Failed to move project")?;
    output::success(&format!(
        "Moved project '{}' to workspace {}",
        moved.project.name, moved.project.workspace_id
    ));
    if moved.unassigned_task_count > 0.0 {
        output::warn(&format!(
            "{} task(s) were left unassigned",
            moved.unassigned_task_count
        ));
    }
    Ok(())
}

/// Sets the sidebar order of a workspace's projects to the given ID order.
pub async fn reorder(
    client: &ApiClient,
    workspace_id: &str,
    order: Vec<String>,
) -> Result<(), KaneoError> {
    let body = reorder_body(order)?;
    let projects: Vec<Project> = client
        .put_query("/project/reorder", &WorkspaceQuery { workspace_id }, &body)
        .await
        .with_action("Failed to reorder projects")?;
    output::success(&format!("Reordered {} project(s)", body.projects.len()));
    for p in &projects {
        println!("{}  ({})  {}", p.name, p.slug, p.id);
    }
    Ok(())
}

/// Builds the reorder body, numbering projects by their position in `order`.
pub fn reorder_body(order: Vec<String>) -> Result<ReorderProjectsBody, KaneoError> {
    let ids: Vec<String> = order
        .into_iter()
        .map(|id| id.trim().to_owned())
        .filter(|id| !id.is_empty())
        .collect();
    if ids.is_empty() {
        return Err(KaneoError::Usage(
            "--order needs at least one project ID".to_owned(),
        ));
    }
    let mut seen: HashSet<&str> = HashSet::new();
    if let Some(duplicate) = ids.iter().find(|id| !seen.insert(id.as_str())) {
        return Err(KaneoError::Usage(format!(
            "Project {duplicate} appears more than once in --order"
        )));
    }
    let projects: Vec<ProjectPosition> = ids
        .into_iter()
        .enumerate()
        .map(|(position, id)| ProjectPosition { id, position })
        .collect();
    Ok(ReorderProjectsBody { projects })
}
