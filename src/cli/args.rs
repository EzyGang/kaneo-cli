use clap::{Args, ValueEnum};

#[derive(Args, Clone)]
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

#[derive(Args, Clone)]
pub struct SetArgs {
    #[arg(long, help = "Project ID to pin")]
    pub project: Option<String>,

    #[arg(long, help = "Write to global config instead of the current directory")]
    pub global: bool,
}

#[derive(Args, Clone)]
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

#[derive(Args, Clone)]
pub struct SearchArgs {
    #[arg(help = "Search query string")]
    pub query: String,

    #[arg(long, value_enum, default_value_t = SearchType::All, help = "Limit search to a resource type")]
    pub r#type: SearchType,

    #[arg(long, help = "Restrict search to a project")]
    pub project_id: Option<String>,

    #[arg(long, default_value_t = 20, help = "Maximum number of results")]
    pub limit: u32,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
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
    #[value(help = "Only activity events")]
    Activities,
}

#[derive(Args, Clone)]
pub struct UpgradeArgs {
    #[arg(long, help = "Reinstall even if already on the latest version")]
    pub force: bool,

    #[arg(long, help = "Install a specific version (e.g. v0.2.0)")]
    pub version: Option<String>,
}

/// Options of `install-skill`, passed on to `skills add`.
#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct InstallSkillArgs {
    #[arg(
        short,
        long,
        help = "Install for the current user instead of the current project"
    )]
    pub global: bool,

    #[arg(
        short,
        long = "agent",
        value_name = "AGENT",
        num_args = 1..,
        help = "Agents to install for, such as claude-code, codex, opencode, cursor, or '*'"
    )]
    pub agents: Vec<String>,

    #[arg(short, long, help = "Skip the installer's confirmations")]
    pub yes: bool,

    #[arg(long, help = "Copy the skill instead of linking it")]
    pub copy: bool,

    #[arg(long, value_enum, default_value_t = Runner::Auto, help = "Package runner that starts the installer")]
    pub runner: Runner,

    #[arg(
        last = true,
        value_name = "SKILLS_OPTIONS",
        help = "More options for `skills add`"
    )]
    pub extra: Vec<String>,
}

/// The package runner that starts the `skills` installer from npm.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Runner {
    #[value(help = "Try npx, then pnpm, then bun")]
    Auto,
    #[value(help = "npx (npm)")]
    Npx,
    #[value(help = "pnpm dlx")]
    Pnpm,
    #[value(help = "bun x")]
    Bun,
}

impl SearchType {
    #[must_use]
    pub fn as_api_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Tasks => "tasks",
            Self::Projects => "projects",
            Self::Workspaces => "workspaces",
            Self::Comments => "comments",
            Self::Activities => "activities",
        }
    }
}
