use crate::api::client::ApiClient;
use crate::api::types::{CreateProjectBody, Project, UpdateProjectBody};
use crate::auth::context;
use crate::cli::projects::ProjectCommand;
use crate::output;

pub async fn run(
    cmd: ProjectCommand,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

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
                .map_err(|e| crate::errors::api_error("Failed to list projects".to_owned(), e))?;

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
            let project: Project = client
                .get(&format!("/project/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Project", &id, e))?;

            println!("{}", project.name);
            println!("  Slug:        {}", project.slug);
            println!("  ID:          {}", project.id);
            println!("  Workspace:   {}", project.workspace_id);
            if let Some(icon) = &project.icon
                && !icon.is_empty()
            {
                println!("  Icon:        {icon}");
            }
            if let Some(desc) = &project.description
                && !desc.is_empty()
            {
                println!("  Description: {desc}");
            }
            println!(
                "  Public:      {}",
                if project.is_public.unwrap_or(false) {
                    "yes"
                } else {
                    "no"
                }
            );
        }

        ProjectCommand::Create {
            name,
            workspace_id,
            slug,
            icon,
            description,
        } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let slug_val = slug.unwrap_or_else(|| slug_from_name(&name));
            let body = CreateProjectBody {
                name,
                workspace_id: ws,
                slug: slug_val,
                icon: icon.unwrap_or_default(),
                description: description.unwrap_or_default(),
            };
            let project: Project = client
                .post("/project", &body)
                .await
                .map_err(|e| crate::errors::api_error("Failed to create project".to_owned(), e))?;

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
            let current: Project = client
                .get(&format!("/project/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Project", &id, e))?;

            let body = UpdateProjectBody {
                name: name.unwrap_or(current.name),
                icon: icon.unwrap_or(current.icon.unwrap_or_default()),
                slug: slug.unwrap_or(current.slug),
                description: description.unwrap_or(current.description.unwrap_or_default()),
                is_public: is_public.unwrap_or(current.is_public.unwrap_or(false)),
            };
            let project: Project = client
                .put(&format!("/project/{id}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("Failed to update project".to_owned(), e))?;

            output::success(&format!("Updated project '{}'", project.name));
        }

        ProjectCommand::Delete { id, force } => {
            if !force {
                eprintln!(
                    "  {} Re-run with `--force` to confirm deletion",
                    output::dim("note:")
                );
                return Ok(());
            }
            let project: Project = client
                .delete(&format!("/project/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Project", &id, e))?;

            output::success(&format!("Deleted project '{}'", project.name));
        }

        ProjectCommand::Archive { id } => {
            let project: Project = client
                .put(&format!("/project/{id}/archive"), &serde_json::json!({}))
                .await
                .map_err(|e| crate::errors::not_found("Project", &id, e))?;

            output::success(&format!("Archived project '{}'", project.name));
        }

        ProjectCommand::Unarchive { id } => {
            let project: Project = client
                .put(&format!("/project/{id}/unarchive"), &serde_json::json!({}))
                .await
                .map_err(|e| crate::errors::not_found("Project", &id, e))?;

            output::success(&format!("Unarchived project '{}'", project.name));
        }
    }

    Ok(())
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
