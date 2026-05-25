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
        limit: u32,
    },

    Get {
        #[arg(help = "Task ID")]
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
        #[arg(help = "Task ID")]
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
        #[arg(help = "Task ID")]
        id: String,

        #[arg(long)]
        force: bool,
    },

    Status {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "New status")]
        status: String,
    },

    Priority {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "New priority")]
        priority: String,
    },

    Assign {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(long)]
        user_id: Option<String>,
    },

    #[command(name = "move")]
    MoveTask {
        #[arg(help = "Task ID")]
        id: String,

        #[arg(help = "Destination project ID")]
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

    #[command(name = "relation")]
    Relation {
        #[command(subcommand)]
        command: TaskRelationCommand,
    },
}

#[derive(Subcommand, Clone)]
pub enum TaskCommentCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(help = "Task ID")]
        task_id: String,
    },

    #[command(name = "add")]
    Add {
        #[arg(help = "Task ID")]
        task_id: String,

        #[arg(help = "Comment text")]
        content: String,
    },

    Update {
        #[arg(help = "Comment ID")]
        id: String,

        #[arg(help = "Updated comment text")]
        content: String,
    },

    #[command(visible_alias = "rm")]
    Delete {
        #[arg(help = "Comment ID")]
        id: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum TaskLabelCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(help = "Task ID")]
        task_id: String,
    },

    Attach {
        #[arg(help = "Task ID")]
        task_id: String,

        #[arg(help = "Label ID")]
        label_id: String,
    },

    Detach {
        #[arg(help = "Task ID")]
        task_id: String,

        #[arg(help = "Label ID")]
        label_id: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum TaskRelationCommand {
    #[command(visible_alias = "ls")]
    List {
        #[arg(help = "Task ID")]
        task_id: String,
    },

    Create {
        #[arg(help = "Source task ID")]
        source_task_id: String,

        #[arg(help = "Target task ID")]
        target_task_id: String,

        #[arg(help = "Relation type (subtask, blocks, related)")]
        relation_type: String,
    },

    #[command(visible_alias = "rm")]
    Delete {
        #[arg(help = "Relation ID")]
        id: String,
    },
}
