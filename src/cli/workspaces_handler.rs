use crate::api::client::ApiClient;
use crate::api::types::Workspace;
use crate::auth::context;
use crate::cli::workspaces::WorkspaceCommand;
use crate::output;

pub async fn run(
    cmd: WorkspaceCommand,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

    match cmd {
        WorkspaceCommand::List => {
            let workspaces: Vec<Workspace> = client
                .get("/auth/organization/list")
                .await
                .map_err(|e| crate::errors::api_error("Failed to list workspaces".to_owned(), e))?;

            if workspaces.is_empty() {
                output::warn("No workspaces found");
                return Ok(());
            }
            for ws in &workspaces {
                println!("{}  ({})  {}", ws.name, ws.slug, ws.id);
            }
        }
    }

    Ok(())
}
