use crate::api::client::ApiClient;
use crate::api::types::project::Project;
use crate::api::types::user::User;
use crate::auth::context::ResolvedContext;
use crate::cli::login;
use crate::cli::task_ref::TaskResolver;
use crate::errors::{ApiResultExt, KaneoError};

/// Everything a task subcommand needs: the API client, the resolved context, and task lookup.
pub struct TaskEnv<'a> {
    pub client: &'a ApiClient,
    pub ctx: &'a ResolvedContext,
    pub tasks: TaskResolver<'a>,
}

impl<'a> TaskEnv<'a> {
    #[must_use]
    pub fn new(client: &'a ApiClient, ctx: &'a ResolvedContext) -> Self {
        Self {
            client,
            ctx,
            tasks: TaskResolver::new(client, ctx.workspace_id.as_deref()),
        }
    }

    /// Returns the slug of a project, which prefixes its ticket IDs.
    pub async fn project_slug(&self, project_id: &str) -> Result<String, KaneoError> {
        let project: Project = self
            .client
            .get(&format!("/project/{project_id}"))
            .await
            .with_action("Failed to get project")?;
        Ok(project.slug)
    }

    /// Returns the ID of the authenticated user.
    pub async fn current_user_id(&self) -> Result<String, KaneoError> {
        let me: User = login::current_user(self.client)
            .await
            .with_action("Failed to get the current user")?;
        Ok(me.id)
    }

    /// Resolves a user argument, where `me` means the authenticated user.
    pub async fn user_id(&self, user: &str) -> Result<String, KaneoError> {
        if user != "me" {
            return Ok(user.to_owned());
        }
        self.current_user_id().await
    }
}
