use clap::{Parser, Subcommand, ValueEnum};

pub mod columns;
pub mod columns_handler;
pub mod labels;
pub mod labels_handler;
pub mod login;
pub mod projects;
pub mod projects_handler;
pub mod search_handler;
pub mod set_unset;
pub mod tasks;
pub mod tasks_handler;
pub mod workspaces;
pub mod workspaces_handler;

use columns::ColumnCommand;
use labels::LabelCommand;
use projects::ProjectCommand;
use tasks::TaskCommand;
use workspaces::WorkspaceCommand;

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

    #[command(
        visible_alias = "ws",
        about = "List workspaces accessible to your API key"
    )]
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommand,
    },

    #[command(about = "Search across tasks, projects, workspaces, and comments")]
    Search(SearchArgs),

    #[command(about = "Download and install the latest version of kaneo")]
    Upgrade(UpgradeArgs),

    #[command(
        name = "install-skill",
        about = "Write a SKILL.md file so AI agents know how to invoke this CLI"
    )]
    InstallSkill(InstallSkillArgs),
}

#[derive(Parser, Clone)]
pub struct LoginArgs {
    #[arg(help = "Your Kaneo API key (find it in Settings → API)")]
    pub api_key: String,

    #[arg(
        long,
        default_value = "cloud.kaneo.app",
        help = "Hostname of the Kaneo instance (e.g. kaneo.example.com)"
    )]
    pub instance: String,
}

#[derive(Parser, Clone)]
pub struct SetArgs {
    #[arg(long, help = "Project ID to pin")]
    pub project: Option<String>,

    #[arg(long, help = "Write to global config instead of the current directory")]
    pub global: bool,
}

#[derive(Parser, Clone)]
pub struct UnsetArgs {
    #[arg(long, help = "Remove only the workspace pin")]
    pub workspace: bool,

    #[arg(long, help = "Remove only the project pin")]
    pub project: bool,

    #[arg(
        long,
        help = "Remove from global config instead of the current directory"
    )]
    pub global: bool,
}

#[derive(Parser, Clone)]
pub struct SearchArgs {
    #[arg(help = "Search query string")]
    pub query: String,

    #[arg(long, value_enum, default_value_t = SearchType::All, help = "Limit search to a resource type")]
    pub r#type: SearchType,

    #[arg(long, help = "Restrict search to a project")]
    pub project_id: Option<String>,

    #[arg(long, default_value = "10", help = "Maximum number of results")]
    pub limit: u32,
}

#[derive(ValueEnum, Clone)]
pub enum SearchType {
    #[value(help = "Everything")]
    All,
    #[value(help = "Only tasks")]
    Tasks,
    #[value(help = "Only projects")]
    Projects,
    #[value(help = "Only workspaces")]
    Workspaces,
    #[value(help = "Only comments")]
    Comments,
}

impl SearchType {
    pub fn as_api_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Tasks => "tasks",
            Self::Projects => "projects",
            Self::Workspaces => "workspaces",
            Self::Comments => "comments",
        }
    }
}

#[derive(Parser, Clone)]
pub struct UpgradeArgs {
    #[arg(long, help = "Reinstall even if already on the latest version")]
    pub force: bool,

    #[arg(long, help = "Install a specific version (e.g. v0.2.0)")]
    pub version: Option<String>,
}

#[derive(Parser, Clone)]
pub struct InstallSkillArgs {
    #[arg(long, value_enum, help = "Which agent to install the skill for")]
    pub agent: InstallSkillAgent,

    #[arg(
        long,
        value_enum,
        help = "Install globally or in the current directory"
    )]
    pub scope: InstallSkillScope,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum InstallSkillAgent {
    #[value(help = "Opencode / Codex CLI")]
    Opencode,
    #[value(help = "Claude Code")]
    Claude,
    #[value(help = "OpenAI Codex CLI (same paths as opencode)")]
    Codex,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum InstallSkillScope {
    #[value(help = "Install globally (~/.config/...)")]
    Global,
    #[value(help = "Install in the current directory")]
    Local,
}
