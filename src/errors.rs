use thiserror::Error;

use crate::api::error::ApiError;

#[derive(Error, Debug)]
pub enum KaneoError {
    #[error("No API key found")]
    MissingApiKey,

    #[error("No workspace configured")]
    MissingWorkspace,

    #[error("No project configured")]
    MissingProject,

    #[error("{action}: {source}")]
    Api {
        action: String,
        #[source]
        source: Box<ApiError>,
    },

    #[error("{0}")]
    Usage(String),

    #[error("{0}")]
    Config(#[source] anyhow::Error),

    #[error("{message}")]
    Upgrade {
        message: String,
        source: anyhow::Error,
    },

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

/// Adds a description of the attempted action to a failed API request.
pub trait ApiResultExt<T> {
    fn with_action(self, action: &str) -> Result<T, KaneoError>;
}

impl<T> ApiResultExt<T> for Result<T, ApiError> {
    fn with_action(self, action: &str) -> Result<T, KaneoError> {
        self.map_err(|source| KaneoError::Api {
            action: action.to_owned(),
            source: Box::new(source),
        })
    }
}

impl KaneoError {
    /// Returns a suggestion for resolving the error, if one applies.
    #[must_use]
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            Self::MissingApiKey => Some("Run `kaneo login <your-api-key>` to authenticate"),
            Self::MissingWorkspace => Some(
                "Provide it with `-w <id>`, `KANEO_WORKSPACE` env var, or `kaneo set -w <workspace-id>`",
            ),
            Self::MissingProject => Some(
                "Provide it with `-p <id>`, `KANEO_PROJECT` env var, or `kaneo set -p <project-id>`",
            ),
            Self::Api { source, .. } => api_hint(source),
            Self::Usage(_) | Self::Config(_) | Self::Upgrade { .. } | Self::Io(_) => None,
        }
    }
}

fn api_hint(error: &ApiError) -> Option<&'static str> {
    match error.code()? {
        "NOT_FOUND" => Some("Use `list` commands to discover available IDs"),
        "UNAUTHORIZED" => Some("Your API key was rejected. Run `kaneo login <your-api-key>`"),
        "MISSING_PERMISSION" | "FORBIDDEN" => {
            Some("Your API key or workspace role lacks the permission for this action")
        }
        "RATE_LIMITED" => Some("The API key hit its rate limit or quota. Retry later"),
        "WORKSPACE_ARCHIVED" => Some("The workspace is archived. Restore it in the admin panel"),
        _ => None,
    }
}

pub fn config_error(source: anyhow::Error) -> KaneoError {
    KaneoError::Config(source)
}

impl From<anyhow::Error> for KaneoError {
    fn from(err: anyhow::Error) -> Self {
        Self::Config(err)
    }
}
