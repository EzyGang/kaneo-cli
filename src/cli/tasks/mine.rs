use clap::Args;
use serde::Serialize;

use crate::api::types::priority::Priority;
use crate::api::types::task_ops::{AssignedTask, AssignedTasks};
use crate::auth::context;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct MineArgs {
    #[arg(long, help = "Workspace ID")]
    pub workspace_id: Option<String>,
    #[arg(long, help = "Print only the number of open tasks assigned to you")]
    pub count: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssignedQuery<'a> {
    workspace_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_only: Option<&'static str>,
}

pub async fn run(args: MineArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let workspace_id = context::require_workspace(args.workspace_id.as_deref(), env.ctx)?;
    let query = AssignedQuery {
        workspace_id: &workspace_id,
        count_only: args.count.then_some("true"),
    };
    let assigned: AssignedTasks = env
        .client
        .get_query("/task/assigned", &query)
        .await
        .with_action("Failed to list your tasks")?;

    if args.count {
        println!("{}", assigned.total);
        return Ok(());
    }
    if assigned.tasks.is_empty() {
        output::warn("No open tasks assigned to you");
        return Ok(());
    }
    print_table(&assigned.tasks);
    eprintln!(
        "\n  {} shown, {} open tasks assigned to you",
        assigned.tasks.len(),
        assigned.total
    );
    Ok(())
}

fn print_table(tasks: &[AssignedTask]) {
    println!(
        "{:<12} {:<42} {:<8} {:<16} {:<10} ID",
        "Ref", "Title", "Priority", "Status", "Due"
    );
    for task in tasks {
        let status = task.status_name.as_deref().unwrap_or(&task.status);
        println!(
            "{:<12} {:<42} {:<8} {:<16} {:<10} {}",
            task.ticket(),
            output::truncate(&task.title, 42),
            Priority::short_label(&task.priority),
            output::truncate(status, 16),
            output::day(task.due_date.as_deref()),
            task.id
        );
    }
}
