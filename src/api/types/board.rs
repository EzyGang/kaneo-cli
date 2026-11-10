use serde::Deserialize;

use crate::api::types::task::Task;

/// One page of `GET /task/tasks/{projectId}`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardPage {
    pub data: BoardData,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardData {
    pub id: String,
    pub name: String,
    pub slug: String,
    #[serde(default)]
    pub workspace_id: Option<String>,
    pub columns: Vec<BoardColumn>,
    #[serde(default)]
    pub planned_tasks: Vec<Task>,
    #[serde(default)]
    pub archived_tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardColumn {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub is_final: bool,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub related_page: u32,
    pub related_page_size: u32,
    pub related_total_pages: u32,
}
