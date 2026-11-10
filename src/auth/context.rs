use crate::api::client::ApiClient;
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
    let instance = std::env::var("KANEO_INSTANCE").unwrap_or(global.instance);

    let api_key = match std::env::var("KANEO_API_KEY") {
        Ok(key) => key,
        Err(_) => global.api_key.ok_or(KaneoError::MissingApiKey)?,
    };

    let local = LocalConfig::merge(&LocalConfig::find_from(
        &std::env::current_dir().unwrap_or_default(),
    ));

    let workspace_id = cli_workspace
        .map(str::to_owned)
        .or_else(|| std::env::var("KANEO_WORKSPACE").ok())
        .or(local.workspace_id)
        .or(global.workspace_id);

    let project_id = cli_project
        .map(str::to_owned)
        .or_else(|| std::env::var("KANEO_PROJECT").ok())
        .or(local.project_id)
        .or(global.project_id);

    Ok(ResolvedContext {
        instance,
        api_key,
        workspace_id,
        project_id,
    })
}

impl ResolvedContext {
    /// Builds an API client for the resolved instance and API key.
    pub fn client(&self) -> Result<ApiClient, KaneoError> {
        ApiClient::new(&self.instance, &self.api_key).map_err(crate::errors::config_error)
    }
}

pub fn require_workspace(
    workspace_id: Option<&str>,
    ctx: &ResolvedContext,
) -> Result<String, KaneoError> {
    workspace_id
        .map(str::to_owned)
        .or_else(|| ctx.workspace_id.clone())
        .ok_or(KaneoError::MissingWorkspace)
}

pub fn require_project(
    project_id: Option<&str>,
    ctx: &ResolvedContext,
) -> Result<String, KaneoError> {
    project_id
        .map(str::to_owned)
        .or_else(|| ctx.project_id.clone())
        .ok_or(KaneoError::MissingProject)
}
