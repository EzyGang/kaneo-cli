use crate::auth::config::GlobalConfig;
use crate::cli::LoginArgs;
use crate::output;

pub async fn run_login(args: LoginArgs) -> anyhow::Result<()> {
    let mut config = GlobalConfig::load()?;
    config.instance = args.instance.clone();
    config.api_key = Some(args.api_key);
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
    output::success("Logged out. Credentials removed.");
    Ok(())
}
