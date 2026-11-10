use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum WorkspaceCommand {
    #[command(visible_alias = "ls", about = "List workspaces you can access")]
    List,

    #[command(about = "Show a workspace")]
    Get {
        #[arg(help = "Workspace ID (default: context workspace)")]
        workspace_id: Option<String>,
    },

    #[command(about = "List members of the context workspace")]
    Members {
        #[arg(long, help = "Only members who can access this project")]
        project_id: Option<String>,
    },

    #[command(about = "Show recent activity across the context workspace, newest first")]
    Activity,
}
