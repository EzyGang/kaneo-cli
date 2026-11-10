use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum NotificationCommand {
    #[command(visible_alias = "ls", about = "List your notifications, newest first")]
    List {
        #[arg(
            long,
            help = "Only notifications for this workspace (default: context, else all)"
        )]
        workspace_id: Option<String>,

        #[arg(long, help = "Only show unread notifications")]
        unread: bool,
    },

    #[command(about = "Mark notifications as read")]
    Read {
        #[arg(required = true, help = "Notification IDs")]
        ids: Vec<String>,
    },

    #[command(about = "Mark a notification as unread")]
    Unread {
        #[arg(help = "Notification ID")]
        id: String,
    },

    #[command(name = "read-all", about = "Mark every notification as read")]
    ReadAll {
        #[arg(
            long,
            help = "Only notifications for this workspace (default: context, else all)"
        )]
        workspace_id: Option<String>,
    },

    #[command(visible_alias = "rm", about = "Delete a notification")]
    Delete {
        #[arg(help = "Notification ID")]
        id: String,
    },

    #[command(about = "Delete all notifications, or only read ones with --read-only")]
    Clear {
        #[arg(long, help = "Only delete notifications that are already read")]
        read_only: bool,

        #[arg(
            long,
            help = "Only notifications for this workspace (default: context, else all)"
        )]
        workspace_id: Option<String>,

        #[arg(long, help = "Confirm the deletion")]
        force: bool,
    },
}
