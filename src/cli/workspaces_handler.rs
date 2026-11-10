use serde::Serialize;

use crate::api::client::ApiClient;
use crate::api::types::activity::WorkspaceActivity;
use crate::api::types::member::WorkspaceMember;
use crate::api::types::workspace::Workspace;
use crate::auth::context::{self, ResolvedContext};
use crate::cli::workspaces::WorkspaceCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Widest activity excerpt shown in a row.
const EXCERPT_WIDTH: usize = 80;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MembersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
}

pub async fn run(cmd: WorkspaceCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        WorkspaceCommand::List => {
            let workspaces: Vec<Workspace> = client
                .get("/workspace")
                .await
                .with_action("Failed to list workspaces")?;
            if workspaces.is_empty() {
                output::warn("No workspaces found");
                return Ok(());
            }
            for ws in &workspaces {
                let role = ws.role.as_deref().unwrap_or("-");
                println!("{}  ({})  {}  {}", ws.name, ws.slug, role, ws.id);
            }
        }

        WorkspaceCommand::Get { workspace_id } => {
            let ws = context::require_workspace(workspace_id.as_deref(), ctx)?;
            let workspace: Workspace = client
                .get(&format!("/workspace/{ws}"))
                .await
                .with_action("Failed to get workspace")?;
            print_workspace(&workspace);
        }

        WorkspaceCommand::Members { project_id } => {
            let ws = context::require_workspace(None, ctx)?;
            members(&client, &ws, project_id).await?;
        }

        WorkspaceCommand::Activity => {
            let ws = context::require_workspace(None, ctx)?;
            activity(&client, &ws).await?;
        }
    }

    Ok(())
}

async fn members(
    client: &ApiClient,
    workspace_id: &str,
    project_id: Option<String>,
) -> Result<(), KaneoError> {
    let members: Vec<WorkspaceMember> = client
        .get_query(
            &format!("/workspace/{workspace_id}/members"),
            &MembersQuery { project_id },
        )
        .await
        .with_action("Failed to list workspace members")?;
    if members.is_empty() {
        output::warn("No members found");
        return Ok(());
    }
    for m in &members {
        println!("{}  {}  {}  {}", m.name, m.email, m.role, m.id);
    }
    Ok(())
}

async fn activity(client: &ApiClient, workspace_id: &str) -> Result<(), KaneoError> {
    let mut activities: Vec<WorkspaceActivity> = client
        .get(&format!("/activity/workspace/{workspace_id}"))
        .await
        .with_action("Failed to list workspace activity")?;
    if activities.is_empty() {
        output::warn("No activity");
        return Ok(());
    }
    activities.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    for a in &activities {
        println!(
            "{}  {}  {}  {}  {}",
            output::minute(&a.created_at),
            a.actor(),
            a.kind,
            a.ticket(),
            output::truncate(&a.summary(), EXCERPT_WIDTH)
        );
    }
    Ok(())
}

fn print_workspace(workspace: &Workspace) {
    println!("{}", workspace.name);
    println!("  Slug:        {}", workspace.slug);
    println!("  ID:          {}", workspace.id);
    println!(
        "  Role:        {}",
        workspace.role.as_deref().unwrap_or("-")
    );
    println!(
        "  Created:     {}",
        output::day(workspace.created_at.as_deref())
    );
    if let Some(desc) = workspace.description.as_deref().filter(|d| !d.is_empty()) {
        println!("  Description: {desc}");
    }
}
