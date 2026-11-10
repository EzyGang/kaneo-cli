use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// The integration event that triggers a workflow rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowEvent {
    #[value(name = "branch_push")]
    BranchPush,
    #[value(name = "pr_opened")]
    PrOpened,
    #[value(name = "pr_merged")]
    PrMerged,
    #[value(name = "issue_opened")]
    IssueOpened,
    #[value(name = "issue_closed")]
    IssueClosed,
    #[value(name = "issue_reopened")]
    IssueReopened,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRule {
    pub id: String,
    pub project_id: String,
    pub integration_type: String,
    pub event_type: String,
    #[serde(default)]
    pub column_id: Option<String>,
    #[serde(default)]
    pub column_name: Option<String>,
    #[serde(default)]
    pub column_slug: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWorkflowRuleBody {
    pub integration_type: String,
    pub event_type: WorkflowEvent,
    pub column_id: String,
}
