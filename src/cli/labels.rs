use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum LabelCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        workspace_id: Option<String>,
    },

    Get {
        #[arg(help = "Label ID")]
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
        #[arg(help = "Label ID")]
        id: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        color: Option<String>,
    },

    #[command(visible_alias = "rm")]
    Delete {
        #[arg(help = "Label ID")]
        id: String,

        #[arg(long)]
        force: bool,
    },
}
