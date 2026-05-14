use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum LabelCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        workspace_id: Option<String>,
    },

    Get {
        id: String,
    },

    Create {
        #[arg(long)]
        name: String,

        #[arg(long)]
        workspace_id: Option<String>,

        #[arg(long, default_value = "#6366f1")]
        color: Option<String>,
    },

    Update {
        id: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        color: Option<String>,
    },

    #[command(visible_alias = "rm")]
    Delete {
        id: String,

        #[arg(long)]
        force: bool,
    },
}
