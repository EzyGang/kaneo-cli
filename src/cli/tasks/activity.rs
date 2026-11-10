use clap::Args;
use serde::Serialize;

use crate::api::types::activity::Activity;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Widest activity summary shown in a row.
const SUMMARY_WIDTH: usize = 80;

#[derive(Args, Clone)]
pub struct ActivityArgs {
    #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
    pub task: String,

    #[arg(
        long,
        value_parser = clap::value_parser!(u8).range(1..=100),
        help = "Only show the N most recent activities (1-100)"
    )]
    pub limit: Option<u8>,
}

#[derive(Serialize)]
struct ActivityQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

pub async fn run(args: ActivityArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let task_id = env.tasks.id(&args.task).await?;
    let mut activities: Vec<Activity> = env
        .client
        .get_query(
            &format!("/activity/{task_id}"),
            &ActivityQuery { limit: args.limit },
        )
        .await
        .with_action("Failed to list task activity")?;
    if activities.is_empty() {
        output::warn("No activity");
        return Ok(());
    }
    activities.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    for a in &activities {
        println!(
            "{}  {}  {}  {}",
            output::minute(&a.created_at),
            a.actor(),
            a.kind,
            output::truncate(&a.summary(), SUMMARY_WIDTH)
        );
    }
    Ok(())
}
