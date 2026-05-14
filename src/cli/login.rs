use crate::api::client::ApiClient;
use crate::api::types::SessionResponse;
use crate::auth::config::GlobalConfig;
use crate::auth::crypto;
use crate::cli::LoginArgs;
use crate::output;

pub async fn run_login(args: LoginArgs) -> anyhow::Result<()> {
    let mut config = GlobalConfig::load()?;
    config.instance = args.instance.clone();

    let encrypted = crypto::encrypt(&args.api_key)?;
    config.api_key = Some(encrypted);
    config.save()?;

    output::success(&format!("Logged in to {}", args.instance));
    Ok(())
}

pub async fn run_logout() -> anyhow::Result<()> {
    let mut config = GlobalConfig::load()?;
    if config.api_key.is_none() {
        output::warn("Not logged in");
        return Ok(());
    }
    config.api_key = None;
    config.workspace_id = None;
    config.project_id = None;
    config.save()?;
    output::success("Logged out");
    Ok(())
}

pub async fn run_whoami(instance: &str, api_key: &str) -> Result<(), crate::errors::KaneoError> {
    let client = ApiClient::new(instance, api_key).map_err(crate::errors::config_error)?;
    let session: SessionResponse = client
        .get("/auth/get-session")
        .await
        .map_err(|e| crate::errors::api_error(format!("failed to get session: {e}"), e))?;

    match session.user {
        Some(user) => {
            output::success(&format!("Logged in as {}", user.name));
            eprintln!("  {} {}", output::dim("id:"), user.id);
            eprintln!("  {} {}", output::dim("email:"), user.email);
        }
        None => {
            output::warn("Not authenticated. Run `kaneo login <api-key>`");
        }
    }

    Ok(())
}
