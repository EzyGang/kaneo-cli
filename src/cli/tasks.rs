use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum TaskCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(long)]
        project_id: Option<String>,

        #[arg(long)]
        status: Option<String>,

        #[arg(long)]
        priority: Option<String>,

        #[arg(long, default_value = "createdAt")]
        sort: Option<String>,

        #[arg(long, default_value = "20")]
        limit: Option<u32>,
    },

    Get {
        id: String,
    },

    Create {
        #[arg(long)]
        title: String,

        #[arg(long)]
        project_id: Option<String>,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        priority: Option<String>,

        #[arg(long)]
        status: Option<String>,

        #[arg(long)]
        due_date: Option<String>,

        #[arg(long)]
        start_date: Option<String>,

        #[arg(long)]
        assignee: Option<String>,
    },

    Update {
        id: String,

        #[arg(long)]
        title: Option<String>,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        priority: Option<String>,

        #[arg(long)]
        status: Option<String>,

        #[arg(long)]
        due_date: Option<String>,

        #[arg(long)]
        start_date: Option<String>,

        #[arg(long)]
        assignee: Option<String>,
    },

    #[command(visible_alias = "rm")]
    Delete {
        id: String,

        #[arg(long)]
        force: bool,
    },

    Status {
        id: String,
        status: String,
    },

    Priority {
        id: String,
        priority: String,
    },

    Assign {
        id: String,

        #[arg(long)]
        user_id: Option<String>,
    },

    #[command(name = "move")]
    MoveTask {
        id: String,
        project_id: String,
    },

    #[command(name = "comment")]
    Comment {
        #[command(subcommand)]
        command: TaskCommentCommand,
    },

    #[command(name = "label")]
    TaskLabel {
        #[command(subcommand)]
        command: TaskLabelCommand,
    },
}

#[derive(Subcommand, Clone)]
pub enum TaskCommentCommand {
    #[command(visible_alias = "ls")]
    List {
        task_id: String,
    },

    #[command(name = "add")]
    Add {
        task_id: String,
        content: String,
    },

    Update {
        id: String,
        content: String,
    },

    #[command(visible_alias = "rm")]
    Delete {
        id: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum TaskLabelCommand {
    #[command(visible_alias = "ls")]
    List {
        task_id: String,
    },

    Attach {
        task_id: String,
        label_id: String,
    },

    Detach {
        task_id: String,
        label_id: String,
    },
}
