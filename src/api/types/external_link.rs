use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// A link from `GET /external-link/task/{taskId}`: a manual link or one managed by an integration.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLink {
    pub id: String,
    /// `url` for a manual link, or an integration resource such as `issue` or `pull_request`.
    pub resource_type: String,
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub integration: Option<LinkIntegration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LinkIntegration {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkResourceType {
    Url,
    PullRequest,
}

/// Pull request status as the CLI exposes it; maps onto the API's `state`/`merged`/`draft` metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum PullRequestStatus {
    Open,
    Closed,
    Merged,
    Draft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    Open,
    Closed,
}

/// The `PullRequestLinkMetadata` object, only accepted on `pull_request` links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PullRequestMetadata {
    pub state: PullRequestState,
    pub merged: bool,
    pub draft: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalLinkBody {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub resource_type: LinkResourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PullRequestMetadata>,
}

#[derive(Debug, Serialize)]
pub struct UpdateExternalLinkBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PullRequestMetadata>,
}

impl ExternalLink {
    /// Returns the pull request status recorded in the link metadata, if any.
    #[must_use]
    pub fn pr_status(&self) -> Option<&str> {
        let metadata = self.metadata.as_ref()?;
        let flag = |key: &str| metadata.get(key).and_then(serde_json::Value::as_bool) == Some(true);
        if flag("merged") {
            return Some("merged");
        }
        if flag("draft") {
            return Some("draft");
        }
        metadata.get("state").and_then(serde_json::Value::as_str)
    }
}

impl PullRequestStatus {
    #[must_use]
    pub fn metadata(self) -> PullRequestMetadata {
        let (state, merged, draft) = match self {
            Self::Open => (PullRequestState::Open, false, false),
            Self::Closed => (PullRequestState::Closed, false, false),
            Self::Merged => (PullRequestState::Closed, true, false),
            Self::Draft => (PullRequestState::Open, false, true),
        };
        PullRequestMetadata {
            state,
            merged,
            draft,
        }
    }
}
