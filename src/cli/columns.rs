use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum ColumnCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        project_id: Option<String>,
    },

    Create {
        #[arg(long)]
        name: String,

        #[arg(long)]
        project_id: Option<String>,

        #[arg(long)]
        icon: Option<String>,

        #[arg(long)]
        color: Option<String>,

        #[arg(long)]
        is_final: bool,
    },

    Update {
        #[arg(help = "Column ID")]
        id: String,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        icon: Option<String>,

        #[arg(long)]
        color: Option<String>,

        #[arg(long)]
        is_final: Option<bool>,
    },

    #[command(visible_alias = "rm")]
    Delete {
        #[arg(help = "Column ID")]
        id: String,

        #[arg(long)]
        force: bool,
    },

    #[command(about = "Set the column order of a project's board")]
    Reorder {
        #[arg(long)]
        project_id: Option<String>,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Comma-separated column IDs in the desired order"
        )]
        order: Vec<String>,
    },
}
