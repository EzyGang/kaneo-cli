use crate::api::client::ApiClient;
use crate::api::types::notification::{Notification, NotificationBulkResult};
use crate::auth::context::ResolvedContext;
use crate::cli::notifications::NotificationCommand;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Widest notification text shown in a list row.
const TEXT_WIDTH: usize = 80;

pub async fn run(cmd: NotificationCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;

    match cmd {
        NotificationCommand::List {
            workspace_id,
            unread,
        } => {
            let path = scoped("/notification", workspace_id.as_deref(), ctx);
            let notifications: Vec<Notification> = client
                .get(&path)
                .await
                .with_action("Failed to list notifications")?;
            let mut shown: Vec<&Notification> = notifications
                .iter()
                .filter(|n| !unread || !n.read())
                .collect();
            shown.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            if shown.is_empty() {
                output::warn("No notifications");
                return Ok(());
            }
            for n in shown {
                let marker = if n.read() { " " } else { "●" };
                println!(
                    "{}  {marker}  {}  {}",
                    output::minute(&n.created_at),
                    output::truncate(&n.text(), TEXT_WIDTH),
                    n.id
                );
            }
        }

        NotificationCommand::Read { ids } => {
            for id in &ids {
                mark(&client, id, "read").await?;
            }
            output::success(&format!("Marked {} notification(s) as read", ids.len()));
        }

        NotificationCommand::Unread { id } => {
            mark(&client, &id, "unread").await?;
            output::success("Marked notification as unread");
        }

        NotificationCommand::ReadAll { workspace_id } => {
            let path = scoped("/notification/read-all", workspace_id.as_deref(), ctx);
            let _: NotificationBulkResult = client
                .patch(&path, &serde_json::json!({}))
                .await
                .with_action("Failed to mark notifications as read")?;
            output::success("Marked all notifications as read");
        }

        NotificationCommand::Delete { id } => {
            let _: NotificationBulkResult = client
                .delete(&format!("/notification/{id}"))
                .await
                .with_action("Failed to delete notification")?;
            output::success("Notification deleted");
        }

        NotificationCommand::Clear {
            read_only,
            workspace_id,
            force,
        } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let (endpoint, which) = if read_only {
                ("/notification/clear-read", "read")
            } else {
                ("/notification/clear-all", "all")
            };
            let path = scoped(endpoint, workspace_id.as_deref(), ctx);
            let _: NotificationBulkResult = client
                .delete(&path)
                .await
                .with_action("Failed to clear notifications")?;
            output::success(&format!("Cleared {which} notifications"));
        }
    }

    Ok(())
}

async fn mark(client: &ApiClient, id: &str, state: &str) -> Result<Notification, KaneoError> {
    client
        .patch(
            &format!("/notification/{id}/{state}"),
            &serde_json::json!({}),
        )
        .await
        .with_action(&format!("Failed to mark notification {id} as {state}"))
}

/// Appends the `workspaceId` filter from the flag or the context; without either, every
/// workspace is included.
fn scoped(path: &str, workspace_id: Option<&str>, ctx: &ResolvedContext) -> String {
    match workspace_id.or(ctx.workspace_id.as_deref()) {
        Some(ws) => format!("{path}?workspaceId={ws}"),
        None => path.to_owned(),
    }
}
