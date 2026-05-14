use crate::auth::config::LocalConfig;
use crate::cli::{SetArgs, UnsetArgs};
use crate::output;

pub fn run_set(args: SetArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = LocalConfig {
        workspace_id: Some(args.workspace_id),
        project_id: args.project,
    };
    LocalConfig::write_to(&cwd, &config)?;
    output::success("Local config written");
    Ok(())
}

pub fn run_unset(args: UnsetArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    if !args.workspace && !args.project {
        let path = cwd.join(".kaneo-conf.json");
        if path.exists() {
            std::fs::remove_file(&path)?;
            output::success("Local config removed");
        } else {
            output::warn("No local config found");
        }
        return Ok(());
    }

    let configs = LocalConfig::find_from(&cwd);
    let existing = LocalConfig::merge(&configs);

    let new_config = LocalConfig {
        workspace_id: if args.workspace {
            None
        } else {
            existing.workspace_id
        },
        project_id: if args.project {
            None
        } else {
            existing.project_id
        },
    };

    if new_config.workspace_id.is_none() && new_config.project_id.is_none() {
        LocalConfig::remove_from(&cwd)?;
    } else {
        LocalConfig::write_to(&cwd, &new_config)?;
    }
    output::success("Local config updated");
    Ok(())
}
