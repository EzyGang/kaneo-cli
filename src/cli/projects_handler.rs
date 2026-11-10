use crate::api::client::ApiClient;
use crate::api::types::project::{CreateProjectBody, Project, UpdateProjectBody};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::project_admin;
use crate::cli::projects::ProjectCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

pub async fn run(cmd: ProjectCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        ProjectCommand::List {
            workspace_id,
            include_archived,
        } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let mut path = format!("/project?workspaceId={ws}");
            if include_archived {
                path.push_str("&includeArchived=true");
            }
            let projects: Vec<Project> = client
                .get(&path)
                .await
                .with_action("Failed to list projects")?;

            if projects.is_empty() {
                output::warn("No projects found");
                return Ok(());
            }
            for p in &projects {
                let archived = if p.archived_at.is_some() {
                    " [archived]"
                } else {
                    ""
                };
                println!("{}  ({})  {}{archived}", p.name, p.slug, p.id);
            }
        }

        ProjectCommand::Get { id } => {
            let project = fetch(&client, &id).await?;
            print_project(&project);
        }

        ProjectCommand::Create {
            name,
            workspace_id,
            slug,
            icon,
            description,
        } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let slug = slug.unwrap_or_else(|| slug_from_name(&name));
            let body = CreateProjectBody {
                name,
                workspace_id: ws,
                slug,
                icon: icon.unwrap_or_else(|| "Layout".to_owned()),
            };
            let mut project: Project = client
                .post("/project", &body)
                .await
                .with_action("Failed to create project")?;

            // The create endpoint has no description field, so it is set with a follow-up replace.
            if let Some(description) = description {
                let id = project.id.clone();
                let body = UpdateProjectBody {
                    description,
                    ..UpdateProjectBody::from_project(project)
                };
                project = replace(&client, &body, &id).await?;
            }
            output::success(&format!(
                "Created project '{}' ({})",
                project.name, project.id
            ));
        }

        ProjectCommand::Update {
            id,
            name,
            icon,
            slug,
            description,
            is_public,
        } => {
            let current = UpdateProjectBody::from_project(fetch(&client, &id).await?);
            let body = UpdateProjectBody {
                name: name.unwrap_or(current.name),
                icon: icon.unwrap_or(current.icon),
                slug: slug.unwrap_or(current.slug),
                description: description.unwrap_or(current.description),
                is_public: is_public.unwrap_or(current.is_public),
            };
            let project = replace(&client, &body, &id).await?;
            output::success(&format!("Updated project '{}'", project.name));
        }

        ProjectCommand::Delete { id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let project: Project = client
                .delete(&format!("/project/{id}"))
                .await
                .with_action("Failed to delete project")?;
            output::success(&format!("Deleted project '{}'", project.name));
        }

        ProjectCommand::Archive { id } => {
            let project = toggle_archive(&client, &id, "archive").await?;
            output::success(&format!("Archived project '{}'", project.name));
        }

        ProjectCommand::Unarchive { id } => {
            let project = toggle_archive(&client, &id, "unarchive").await?;
            output::success(&format!("Unarchived project '{}'", project.name));
        }

        ProjectCommand::MoveProject { id, to, force } => {
            project_admin::move_project(&client, &id, to, force).await?;
        }

        ProjectCommand::Reorder {
            order,
            workspace_id,
        } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            project_admin::reorder(&client, &ws, order).await?;
        }
    }

    Ok(())
}

async fn fetch(client: &ApiClient, id: &str) -> Result<Project, KaneoError> {
    client
        .get(&format!("/project/{id}"))
        .await
        .with_action("Failed to get project")
}

async fn replace(
    client: &ApiClient,
    body: &UpdateProjectBody,
    id: &str,
) -> Result<Project, KaneoError> {
    client
        .put(&format!("/project/{id}"), body)
        .await
        .with_action("Failed to update project")
}

async fn toggle_archive(client: &ApiClient, id: &str, action: &str) -> Result<Project, KaneoError> {
    client
        .put(&format!("/project/{id}/{action}"), &serde_json::json!({}))
        .await
        .with_action(&format!("Failed to {action} project"))
}

fn print_project(project: &Project) {
    println!("{}", project.name);
    println!("  Slug:        {}", project.slug);
    println!("  ID:          {}", project.id);
    println!("  Workspace:   {}", project.workspace_id);
    if let Some(icon) = project.icon.as_deref().filter(|i| !i.is_empty()) {
        println!("  Icon:        {icon}");
    }
    if let Some(desc) = project.description.as_deref().filter(|d| !d.is_empty()) {
        println!("  Description: {desc}");
    }
    let public = if project.is_public.unwrap_or(false) {
        "yes"
    } else {
        "no"
    };
    println!("  Public:      {public}");
    if project.archived_at.is_some() {
        println!("  Archived:    yes");
    }
}

fn slug_from_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
