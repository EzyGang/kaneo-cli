pub mod api;
mod auth;
mod cli;
mod errors;
mod output;
mod skill;
mod upgrade;

#[cfg(test)]
mod skill_tests;

use clap::Parser;

use crate::auth::context::{ResolvedContext, resolve_context};
use crate::cli::{Cli, Command};
use crate::errors::KaneoError;

pub async fn run() {
    match run_cli().await {
        Ok(()) => (),
        Err(err) => {
            let red = console::style("Error:").red().bold();
            eprintln!("  {red} {err}");
            if let Some(hint) = err.hint() {
                let dim = console::style(format!("Hint: {hint}")).dim();
                eprintln!("  {dim}");
            }
            std::process::exit(1);
        }
    }
}

async fn run_cli() -> Result<(), KaneoError> {
    let cli = Cli::parse();

    tokio::spawn(async {
        upgrade::spawn_version_check();
    });

    let is_upgrade = matches!(cli.command, Command::Upgrade(_));
    let ctx = || -> Result<ResolvedContext, KaneoError> {
        resolve_context(cli.workspace.as_deref(), cli.project.as_deref())
    };

    match cli.command {
        Command::Login(args) => cli::login::run_login(args).await?,
        Command::Logout => cli::login::run_logout().await?,
        Command::Whoami => cli::login::run_whoami(&ctx()?).await?,
        Command::Set(args) => cli::set_unset::run_set(args, &ctx()?)?,
        Command::Unset(args) => cli::set_unset::run_unset(args)?,
        Command::Project { command } => cli::projects_handler::run(command, &ctx()?).await?,
        Command::Task { command } => cli::tasks::handler::run(command, &ctx()?).await?,
        Command::Column { command } => cli::columns_handler::run(command, &ctx()?).await?,
        Command::Label { command } => cli::labels_handler::run(command, &ctx()?).await?,
        Command::Field { command } => cli::fields_handler::run(command, &ctx()?).await?,
        Command::Workflow { command } => cli::workflows_handler::run(command, &ctx()?).await?,
        Command::Workspace { command } => cli::workspaces_handler::run(command, &ctx()?).await?,
        Command::Notification { command } => {
            cli::notifications_handler::run(command, &ctx()?).await?
        }
        Command::Search(args) => cli::search_handler::run(args, &ctx()?).await?,
        Command::Time { command } => cli::time_handler::run(command, &ctx()?).await?,
        Command::Upgrade(args) => upgrade::run(args.force, args.version).await?,
        Command::Skill => skill::print()?,
        Command::InstallSkill(args) => skill::install(&args)?,
    }

    if !is_upgrade && let Some(version) = upgrade::check_cached_update() {
        let dim = console::style(format!(
            "  Update available: v{version}. Run `kaneo upgrade`"
        ))
        .color256(245);
        eprintln!();
        eprintln!("{dim}");
    }

    Ok(())
}
