pub mod api;
mod auth;
mod cli;
mod errors;
mod install_skill;
mod output;
mod upgrade;

use clap::Parser;
use cli::{Cli, Command};
use errors::KaneoError;

pub async fn run() {
    match run_cli().await {
        Ok(()) => (),
        Err(err) => {
            let red = console::style("Error:").red().bold();
            eprintln!("  {red} {err}");
            print_hint(&err);
            std::process::exit(1);
        }
    }
}

fn print_hint(err: &KaneoError) {
    let hint = match err {
        KaneoError::MissingApiKey { hint, .. }
        | KaneoError::MissingWorkspace { hint, .. }
        | KaneoError::MissingProject { hint, .. }
        | KaneoError::NotFound { hint, .. } => Some(*hint),
        _ => None,
    };

    if let Some(hint) = hint {
        let dim = console::style(format!("Hint: {hint}")).dim();
        eprintln!("  {dim}");
    }
}

async fn run_cli() -> Result<(), KaneoError> {
    let cli = Cli::parse();

    tokio::spawn(async {
        upgrade::spawn_version_check();
    });

    match &cli.command {
        Command::Login(args) => {
            cli::login::run_login(args.clone()).await?;
        }

        Command::Logout => {
            cli::login::run_logout().await?;
        }

        Command::Set(args) => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::set_unset::run_set(args.clone(), &ctx)?;
        }

        Command::Unset(args) => {
            cli::set_unset::run_unset(args.clone())?;
        }

        Command::Project { command } => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::projects_handler::run(command.clone(), &ctx).await?;
        }

        Command::Task { command } => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::tasks_handler::run(command.clone(), &ctx).await?;
        }

        Command::Column { command } => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::columns_handler::run(command.clone(), &ctx).await?;
        }

        Command::Label { command } => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::labels_handler::run(command.clone(), &ctx).await?;
        }

        Command::Search(args) => {
            let ctx =
                auth::context::resolve_context(cli.workspace.as_deref(), cli.project.as_deref())?;
            cli::search_handler::run(args.clone(), &ctx).await?;
        }

        Command::Upgrade(args) => {
            upgrade::run(args.force, args.version.clone()).await?;
        }

        Command::InstallSkill(args) => {
            install_skill::run(args.agent, args.scope)?;
        }
    }

    if !matches!(&cli.command, Command::Upgrade(_))
        && let Some(version) = upgrade::check_cached_update()
    {
        let dim = console::style(format!(
            "  Update available: v{version}. Run `kaneo upgrade`"
        ))
        .color256(245);
        eprintln!();
        eprintln!("{dim}");
    }

    Ok(())
}
