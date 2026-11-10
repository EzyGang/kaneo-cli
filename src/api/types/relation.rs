use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRelation {
    pub id: String,
    pub source_task_id: String,
    pub target_task_id: String,
    pub relation_type: String,
    pub created_at: String,
    #[serde(default)]
    pub source_task: Option<RelatedTask>,
    #[serde(default)]
    pub target_task: Option<RelatedTask>,
}

/// A summary of a task on one end of a relation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedTask {
    pub id: String,
    pub title: String,
    pub status: String,
    pub is_completed: bool,
    #[serde(default)]
    pub number: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum RelationType {
    Subtask,
    Blocks,
    Related,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRelationBody {
    pub source_task_id: String,
    pub target_task_id: String,
    pub relation_type: RelationType,
}

impl fmt::Display for RelatedTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let done = if self.is_completed { " ✓" } else { "" };
        match self.number {
            Some(number) => write!(f, "#{number} {}{done}", self.title),
            None => write!(f, "{}{done}", self.title),
        }
    }
}
