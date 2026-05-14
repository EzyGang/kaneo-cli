use crate::auth::config::LocalConfig;
use crate::auth::context;
use crate::cli::{SetArgs, UnsetArgs};
use crate::output;

pub fn run_set(args: SetArgs, ctx: &context::ResolvedContext) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let workspace_id = ctx.workspace_id.clone().or_else(|| {
        eprintln!(
            "  {} Provide a workspace with `-w <id>` or `kaneo set <workspace-id>`",
            output::dim("note:")
        );
        None
    });

    if workspace_id.is_none() && args.project.is_none() {
        output::warn("No workspace or project specified. Use `-w <id>` or `--project <id>`.");
        return Ok(());
    }

    let config = LocalConfig {
        workspace_id,
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
