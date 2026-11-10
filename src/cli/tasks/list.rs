use clap::Args;

use crate::api::board::{BoardEntry, BoardFilters, BoardRequest, Bucket, SortField, SortOrder};
use crate::api::types::priority::Priority;
use crate::auth::context;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone)]
pub struct ListArgs {
    #[arg(long, help = "Project ID")]
    pub project_id: Option<String>,

    #[arg(
        long,
        help = "Only tasks in this column, by slug such as `in-progress`"
    )]
    pub status: Option<String>,

    #[arg(long, value_enum, help = "Only tasks with this priority")]
    pub priority: Option<Priority>,

    #[arg(long, help = "Only tasks assigned to this user ID, or `me`")]
    pub assignee: Option<String>,

    #[arg(long, value_enum, help = "Sort field")]
    pub sort_by: Option<SortField>,

    #[arg(long, value_enum, help = "Sort direction")]
    pub order: Option<SortOrder>,

    #[arg(long, help = "Only tasks due on or before this date")]
    pub due_before: Option<String>,

    #[arg(long, help = "Only tasks due on or after this date")]
    pub due_after: Option<String>,

    #[arg(long, default_value_t = 50, help = "Maximum number of tasks to show")]
    pub limit: usize,

    #[arg(long, help = "Show every matching task, ignoring --limit")]
    pub all: bool,

    #[arg(
        long,
        help = "Include tasks in final (done) columns and archived tasks"
    )]
    pub include_done: bool,
}

pub async fn run(args: ListArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let pid = context::require_project(args.project_id.as_deref(), env.ctx)?;
    let assignee_id = match args.assignee.as_deref() {
        Some(user) => Some(env.user_id(user).await?),
        None => None,
    };
    let request = BoardRequest {
        filters: BoardFilters {
            status: args.status,
            priority: args.priority,
            assignee_id,
            sort_by: args.sort_by,
            sort_order: args.order,
            due_before: args.due_before,
            due_after: args.due_after,
        },
        cap: (!args.all).then_some(args.limit),
        include_done: args.include_done,
    };
    let board = env
        .client
        .load_board(&pid, &request)
        .await
        .with_action("Failed to list tasks")?;

    if board.entries.is_empty() {
        output::warn("No tasks found");
        return Ok(());
    }

    print_table(&board.project_slug, &board.entries);
    let scope = if args.include_done {
        ""
    } else {
        " (done hidden)"
    };
    eprintln!(
        "\n  {} shown, {} matching{scope}",
        board.entries.len(),
        board.total
    );
    Ok(())
}

fn print_table(slug: &str, entries: &[BoardEntry]) {
    println!(
        "{:<12} {:<42} {:<8} {:<14} {:<16} {:<10} ID",
        "Ref", "Title", "Priority", "Status", "Assignee", "Due"
    );
    for entry in entries {
        let task = &entry.task;
        let status = match &entry.bucket {
            Bucket::Column { .. } => task.status.as_str(),
            Bucket::Planned => "planned",
            Bucket::Archived => "archived",
        };
        println!(
            "{:<12} {:<42} {:<8} {:<14} {:<16} {:<10} {}",
            task.ticket(slug),
            output::truncate(&task.title, 42),
            Priority::short_label(&task.priority),
            status,
            output::truncate(task.assignee_name.as_deref().unwrap_or("-"), 16),
            output::day(task.due_date.as_deref()),
            task.id
        );
    }
}
