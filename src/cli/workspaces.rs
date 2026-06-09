use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum WorkspaceCommand {
    #[command(visible_alias = "ls")]
    List,
}
