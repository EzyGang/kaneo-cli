use crate::api::client::ApiClient;
use crate::api::error::ApiError;
use crate::api::types::user::User;
use crate::auth::config::GlobalConfig;
use crate::auth::context::ResolvedContext;
use crate::cli::args::LoginArgs;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Fetches the user the API key belongs to.
pub async fn current_user(client: &ApiClient) -> Result<User, ApiError> {
    client.get("/user/me").await
}

/// Saves the credentials, then checks them against the instance. A failed check only warns, so
/// the CLI can be configured while the instance is unreachable.
pub async fn run_login(args: LoginArgs) -> anyhow::Result<()> {
    let mut config = GlobalConfig::load()?;
    config.instance = args.instance.clone();
    config.api_key = Some(args.api_key.clone());
    config.save()?;

    match verify(&args.instance, &args.api_key).await {
        Ok(user) => output::success(&format!(
            "Logged in to {} as {} ({})",
            args.instance, user.name, user.email
        )),
        Err(err) => {
            output::success(&format!("Saved credentials for {}", args.instance));
            output::warn(&format!("Could not verify the API key: {err}"));
        }
    }
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

pub async fn run_whoami(ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;
    let user = current_user(&client)
        .await
        .with_action("Failed to get the current user")?;
    println!("{}", user.name);
    println!("  Email:    {}", user.email);
    println!("  Role:     {}", user.role.as_deref().unwrap_or("-"));
    println!("  User ID:  {}", user.id);
    println!("  Instance: {}", ctx.instance);
    Ok(())
}

async fn verify(instance: &str, api_key: &str) -> Result<User, String> {
    let client = ApiClient::new(instance, api_key).map_err(|err| format!("{err:#}"))?;
    current_user(&client).await.map_err(|err| err.to_string())
}
