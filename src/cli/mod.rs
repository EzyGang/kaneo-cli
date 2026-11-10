pub mod args;
pub mod columns;
pub mod columns_handler;
pub mod duration;
pub mod field_value;
pub mod fields;
pub mod fields_handler;
pub mod labels;
pub mod labels_handler;
pub mod login;
pub mod notifications;
pub mod notifications_handler;
pub mod project_admin;
pub mod projects;
pub mod projects_handler;
pub mod search_handler;
pub mod set_unset;
pub mod task_ref;
pub mod tasks;
pub mod time;
pub mod time_api;
pub mod time_clock;
pub mod time_handler;
pub mod workflows;
pub mod workflows_handler;
pub mod workspaces;
pub mod workspaces_handler;

#[cfg(test)]
mod duration_tests;
#[cfg(test)]
mod field_value_tests;
#[cfg(test)]
mod project_admin_tests;
#[cfg(test)]
mod task_ref_tests;

use clap::{Parser, Subcommand};

use crate::cli::args::{InstallSkillArgs, LoginArgs, SearchArgs, SetArgs, UnsetArgs, UpgradeArgs};
use crate::cli::columns::ColumnCommand;
use crate::cli::fields::FieldCommand;
use crate::cli::labels::LabelCommand;
use crate::cli::notifications::NotificationCommand;
use crate::cli::projects::ProjectCommand;
use crate::cli::tasks::command::TaskCommand;
use crate::cli::time::TimeCommand;
use crate::cli::workflows::WorkflowCommand;
use crate::cli::workspaces::WorkspaceCommand;

#[derive(Parser)]
#[command(
    name = "kaneo",
    version,
    about = "A minimalist CLI for the Kaneo project management tool.",
    long_about = "A minimalist CLI for the Kaneo project management tool.\n\nAuthenticate once, then manage projects, tasks, columns, labels, and\ncomments from your terminal."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        short = 'w',
        long,
        global = true,
        env = "KANEO_WORKSPACE",
        help = "Override the workspace ID for this command"
    )]
    pub workspace: Option<String>,

    #[arg(
        short = 'p',
        long,
        global = true,
        env = "KANEO_PROJECT",
        help = "Override the project ID for this command"
    )]
    pub project: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Store an API key to authenticate with Kaneo")]
    Login(LoginArgs),

    #[command(about = "Remove stored credentials")]
    Logout,

    #[command(about = "Show the authenticated user and instance")]
    Whoami,

    #[command(
        name = "set",
        about = "Pin workspace and project in the current directory"
    )]
    Set(SetArgs),

    #[command(name = "unset", about = "Remove per-directory workspace/project pins")]
    Unset(UnsetArgs),

    #[command(
        visible_alias = "proj",
        about = "Manage projects — create, list, update, archive, and delete"
    )]
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },

    #[command(
        name = "task",
        about = "Manage tasks — list, create, update, assign, comment, and label"
    )]
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },

    #[command(
        visible_alias = "col",
        about = "Manage board columns — create, list, reorder, and delete"
    )]
    Column {
        #[command(subcommand)]
        command: ColumnCommand,
    },

    #[command(about = "Manage workspace labels — create, list, update, and delete")]
    Label {
        #[command(subcommand)]
        command: LabelCommand,
    },

    #[command(about = "Manage custom fields of a project")]
    Field {
        #[command(subcommand)]
        command: FieldCommand,
    },

    #[command(about = "Manage workflow rules that move tasks on integration events")]
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommand,
    },

    #[command(visible_alias = "ws", about = "List and inspect workspaces")]
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommand,
    },

    #[command(visible_alias = "inbox", about = "Read and manage your notifications")]
    Notification {
        #[command(subcommand)]
        command: NotificationCommand,
    },

    #[command(about = "Search across tasks, projects, workspaces, comments, and activity")]
    Search(SearchArgs),

    #[command(about = "Track time on tasks")]
    Time {
        #[command(subcommand)]
        command: TimeCommand,
    },

    #[command(about = "Download and install the latest version of kaneo")]
    Upgrade(UpgradeArgs),

    #[command(about = "Print the agent skill (SKILL.md) that explains this CLI")]
    Skill,

    #[command(
        name = "install-skill",
        about = "Install the agent skill into AI agents with the npm `skills` installer",
        after_help = INSTALL_SKILL_HELP
    )]
    InstallSkill(InstallSkillArgs),
}

const INSTALL_SKILL_HELP: &str = "\
Examples:
  kaneo install-skill                          asks for the agents and the scope
  kaneo install-skill -g -a claude-code -y     Claude Code, for this user
  kaneo install-skill -a '*' -y                every agent, in this project
  kaneo install-skill --runner bun             use bun instead of npx/pnpm

Without Node.js or bun, save the skill yourself:
  mkdir -p ~/.agents/skills/kaneo && kaneo skill > ~/.agents/skills/kaneo/SKILL.md";
