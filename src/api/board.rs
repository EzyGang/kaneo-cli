use clap::ValueEnum;
use serde::Serialize;

use crate::api::client::ApiClient;
use crate::api::error::ApiError;
use crate::api::types::board::{BoardData, BoardPage};
use crate::api::types::priority::Priority;
use crate::api::types::task::Task;

/// The largest page the board endpoint returns.
const MAX_PAGE_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum SortField {
    CreatedAt,
    Priority,
    DueDate,
    Position,
    Title,
    Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Server-side filters of `GET /task/tasks/{projectId}`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_after: Option<String>,
}

/// What to load from a project board.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BoardRequest {
    pub filters: BoardFilters,
    /// Maximum number of tasks to keep, or `None` for every task.
    pub cap: Option<usize>,
    /// Keep tasks in final columns and archived tasks.
    pub include_done: bool,
}

/// Every page of a board merged into one task list.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedBoard {
    pub project_slug: String,
    pub entries: Vec<BoardEntry>,
    /// Tasks matching the server-side filters, including done ones.
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardEntry {
    pub task: Task,
    pub bucket: Bucket,
}

/// Where a task sits on the board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bucket {
    Column { name: String, is_final: bool },
    Planned,
    Archived,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
struct PageQuery<'a> {
    #[serde(flatten)]
    filters: &'a BoardFilters,
    page: u32,
    limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_page: Option<u32>,
}

impl ApiClient {
    /// Loads a project board, following task pages and the related-data pages of each.
    pub async fn load_board(
        &self,
        project_id: &str,
        request: &BoardRequest,
    ) -> Result<LoadedBoard, ApiError> {
        let path = format!("/task/tasks/{project_id}");
        let limit = request
            .cap
            .map_or(MAX_PAGE_SIZE, |cap| cap.clamp(1, MAX_PAGE_SIZE));
        let mut query = PageQuery {
            filters: &request.filters,
            page: 1,
            limit,
            related_page: None,
        };
        let mut page: BoardPage = self.get_query(&path, &query).await?;
        let mut board = LoadedBoard::empty(&page.data, page.pagination.total);
        loop {
            let pagination = page.pagination;
            board.absorb(page.data, request.include_done);
            for related_page in 2..=pagination.related_total_pages {
                let related = PageQuery {
                    related_page: Some(related_page),
                    ..query
                };
                let extra: BoardPage = self.get_query(&path, &related).await?;
                board.merge_labels(extra.data);
            }
            let full = request.cap.is_some_and(|cap| board.entries.len() >= cap);
            if full || query.page >= pagination.total_pages {
                break;
            }
            query.page += 1;
            page = self.get_query(&path, &query).await?;
        }
        if let Some(cap) = request.cap {
            board.entries.truncate(cap);
        }
        Ok(board)
    }
}

impl LoadedBoard {
    #[must_use]
    pub fn empty(data: &BoardData, total: u64) -> Self {
        Self {
            project_slug: data.slug.clone(),
            entries: Vec::new(),
            total,
        }
    }

    /// Appends the tasks of one page, dropping done tasks unless `include_done` is set.
    pub fn absorb(&mut self, data: BoardData, include_done: bool) {
        let columns = data.columns.into_iter().flat_map(|column| {
            let bucket = Bucket::Column {
                name: column.name,
                is_final: column.is_final,
            };
            column.tasks.into_iter().map(move |task| BoardEntry {
                task,
                bucket: bucket.clone(),
            })
        });
        let planned = data.planned_tasks.into_iter().map(|task| BoardEntry {
            task,
            bucket: Bucket::Planned,
        });
        let archived = data.archived_tasks.into_iter().map(|task| BoardEntry {
            task,
            bucket: Bucket::Archived,
        });
        self.entries.extend(
            columns
                .chain(planned)
                .chain(archived)
                .filter(|entry| include_done || !entry.bucket.is_done()),
        );
    }

    /// Adds labels delivered on a related-data page to the tasks already loaded.
    pub fn merge_labels(&mut self, extra: BoardData) {
        let extra_tasks = extra
            .columns
            .into_iter()
            .flat_map(|column| column.tasks)
            .chain(extra.planned_tasks)
            .chain(extra.archived_tasks);
        for extra_task in extra_tasks.filter(|task| !task.labels.is_empty()) {
            if let Some(entry) = self.entries.iter_mut().find(|e| e.task.id == extra_task.id) {
                entry.task.labels.extend(extra_task.labels);
            }
        }
    }
}

impl Bucket {
    /// Returns whether tasks here count as finished.
    #[must_use]
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Column { is_final: true, .. } | Self::Archived)
    }
}
