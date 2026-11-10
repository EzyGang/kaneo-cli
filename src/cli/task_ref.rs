use serde::Serialize;

use crate::api::client::ApiClient;
use crate::api::types::task::Task;
use crate::errors::{ApiResultExt, KaneoError};

/// A user-supplied task reference: a ticket ID such as `KAN-12`, or a raw task ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskRef<'a> {
    Ticket(&'a str),
    Id(&'a str),
}

/// Resolves task references against the API, scoped to the active workspace.
pub struct TaskResolver<'a> {
    client: &'a ApiClient,
    workspace_id: Option<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TicketQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_id: Option<&'a str>,
}

impl<'a> TaskRef<'a> {
    /// Classifies `input` as a ticket ID when it looks like `<project key>-<number>`.
    #[must_use]
    pub fn parse(input: &'a str) -> Self {
        let input = input.trim();
        let Some((key, number)) = input.rsplit_once('-') else {
            return Self::Id(input);
        };
        let key_ok = key.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
            && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
        let number_ok =
            !number.is_empty() && number.len() <= 9 && number.chars().all(|c| c.is_ascii_digit());
        if !key_ok || !number_ok {
            return Self::Id(input);
        }
        Self::Ticket(input)
    }
}

impl<'a> TaskResolver<'a> {
    #[must_use]
    pub fn new(client: &'a ApiClient, workspace_id: Option<&'a str>) -> Self {
        Self {
            client,
            workspace_id,
        }
    }

    /// Fetches the task that `input` refers to.
    pub async fn task(&self, input: &str) -> Result<Task, KaneoError> {
        match TaskRef::parse(input) {
            TaskRef::Ticket(ticket) => {
                let query = TicketQuery {
                    workspace_id: self.workspace_id,
                };
                self.client
                    .get_query(
                        &format!("/task/by-ticket-id/{}", ticket.to_uppercase()),
                        &query,
                    )
                    .await
                    .with_action(&format!("Failed to find task {ticket}"))
            }
            TaskRef::Id(id) => self
                .client
                .get(&format!("/task/{id}"))
                .await
                .with_action(&format!("Failed to get task {id}")),
        }
    }

    /// Returns the task ID that `input` refers to, looking up ticket IDs.
    pub async fn id(&self, input: &str) -> Result<String, KaneoError> {
        match TaskRef::parse(input) {
            TaskRef::Ticket(_) => Ok(self.task(input).await?.id),
            TaskRef::Id(id) => Ok(id.to_owned()),
        }
    }
}
