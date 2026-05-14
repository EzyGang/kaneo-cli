use crate::auth::config::{GlobalConfig, LocalConfig};
use crate::errors::KaneoError;

pub struct ResolvedContext {
    pub instance: String,
    pub api_key: String,
    pub workspace_id: Option<String>,
    pub project_id: Option<String>,
}

pub fn resolve_context(
    cli_workspace: Option<&str>,
    cli_project: Option<&str>,
) -> Result<ResolvedContext, KaneoError> {
    let global = GlobalConfig::load().map_err(crate::errors::config_error)?;
    let instance = std::env::var("KANEO_INSTANCE")
        .ok()
        .unwrap_or(global.instance.clone());
    let global_workspace = global.workspace_id.clone();
    let global_project = global.project_id.clone();

    let api_key = match std::env::var("KANEO_API_KEY") {
        Ok(key) => key,
        Err(_) => global
            .decrypted_api_key()?
            .ok_or_else(crate::errors::missing_api_key)?,
    };

    let workspace_id = cli_workspace
        .map(|s| s.to_owned())
        .or_else(|| std::env::var("KANEO_WORKSPACE").ok())
        .or_else(|| {
            let cwd = std::env::current_dir().unwrap_or_default();
            let local_configs = LocalConfig::find_from(&cwd);
            let merged = LocalConfig::merge(&local_configs);
            merged.workspace_id
        })
        .or(global_workspace);

    let project_id = cli_project
        .map(|s| s.to_owned())
        .or_else(|| std::env::var("KANEO_PROJECT").ok())
        .or_else(|| {
            let cwd = std::env::current_dir().unwrap_or_default();
            let local_configs = LocalConfig::find_from(&cwd);
            let merged = LocalConfig::merge(&local_configs);
            merged.project_id
        })
        .or(global_project);

    Ok(ResolvedContext {
        instance,
        api_key,
        workspace_id,
        project_id,
    })
}

pub fn require_workspace(
    workspace_id: Option<&str>,
    ctx: &ResolvedContext,
) -> Result<String, KaneoError> {
    workspace_id
        .map(|s| s.to_owned())
        .or_else(|| ctx.workspace_id.clone())
        .ok_or_else(crate::errors::missing_workspace)
}

pub fn require_project(
    project_id: Option<&str>,
    ctx: &ResolvedContext,
) -> Result<String, KaneoError> {
    project_id
        .map(|s| s.to_owned())
        .or_else(|| ctx.project_id.clone())
        .ok_or_else(crate::errors::missing_project)
}
