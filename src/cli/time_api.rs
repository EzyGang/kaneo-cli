use crate::api::types::time_entry::{CreateTimeEntryBody, TimeEntry, TimeEntryBody};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};

pub async fn fetch(env: &TaskEnv<'_>, id: &str) -> Result<TimeEntry, KaneoError> {
    env.client
        .get(&format!("/time-entry/{id}"))
        .await
        .with_action("Failed to get time entry")
}

pub async fn entries(env: &TaskEnv<'_>, task_id: &str) -> Result<Vec<TimeEntry>, KaneoError> {
    env.client
        .get(&format!("/time-entry/task/{task_id}"))
        .await
        .with_action("Failed to list time entries")
}

/// Returns the authenticated user's running entry on a task.
pub async fn running_entry(
    env: &TaskEnv<'_>,
    task_id: &str,
) -> Result<Option<TimeEntry>, KaneoError> {
    let (me, entries) = tokio::try_join!(env.current_user_id(), entries(env, task_id))?;
    Ok(entries
        .into_iter()
        .find(|e| e.is_running() && e.user_id.as_deref() == Some(me.as_str())))
}

pub async fn create(
    env: &TaskEnv<'_>,
    task_id: String,
    entry: TimeEntryBody,
) -> Result<TimeEntry, KaneoError> {
    env.client
        .post("/time-entry", &CreateTimeEntryBody { task_id, entry })
        .await
        .with_action("Failed to create time entry")
}

/// Replaces every field of a time entry.
pub async fn save(
    env: &TaskEnv<'_>,
    id: &str,
    body: &TimeEntryBody,
) -> Result<TimeEntry, KaneoError> {
    env.client
        .put(&format!("/time-entry/{id}"), body)
        .await
        .with_action("Failed to update time entry")
}
