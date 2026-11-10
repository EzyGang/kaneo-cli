use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum ProjectCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        workspace_id: Option<String>,

        #[arg(long)]
        include_archived: bool,
    },

    Get {
        #[arg(help = "Project ID")]
        id: String,
    },

    Create {
        #[arg(long)]
        name: String,

        #[arg(long)]
        workspace_id: Option<String>,

        #[arg(long)]
        slug: Option<String>,

        #[arg(long)]
        icon: Option<String>,

        #[arg(long)]
        description: Option<String>,
    },

    Update {
        #[arg(help = "Project ID")]
        id: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        icon: Option<String>,

        #[arg(long)]
        slug: Option<String>,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        is_public: Option<bool>,
    },

    #[command(visible_alias = "rm")]
    Delete {
        #[arg(help = "Project ID")]
        id: String,

        #[arg(long)]
        force: bool,
    },

    Archive {
        #[arg(help = "Project ID")]
        id: String,
    },

    Unarchive {
        #[arg(help = "Project ID")]
        id: String,
    },

    #[command(
        name = "move",
        about = "Move a project and its tasks to another workspace"
    )]
    MoveProject {
        #[arg(help = "Project ID")]
        id: String,

        #[arg(long = "to", help = "Destination workspace ID")]
        to: String,

        #[arg(long, help = "Confirm the move")]
        force: bool,
    },

    #[command(about = "Set the sidebar order of a workspace's projects")]
    Reorder {
        #[arg(
            long,
            required = true,
            value_delimiter = ',',
            help = "Project IDs in the desired order, comma-separated"
        )]
        order: Vec<String>,

        #[arg(long)]
        workspace_id: Option<String>,
    },
}
