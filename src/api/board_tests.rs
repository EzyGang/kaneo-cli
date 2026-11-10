use serde_json::json;

use crate::api::board::{Bucket, LoadedBoard};
use crate::api::types::board::BoardData;

fn task(id: &str, labels: serde_json::Value) -> serde_json::Value {
    json!({
        "id": id, "projectId": "p", "title": id, "status": "s", "priority": "low",
        "createdAt": "2026-01-01", "labels": labels
    })
}

fn data(todo: &[&str], done: &[&str], planned: &[&str], archived: &[&str]) -> BoardData {
    let tasks = |ids: &[&str]| ids.iter().map(|id| task(id, json!([]))).collect::<Vec<_>>();
    serde_json::from_value(json!({
        "id": "p", "name": "Project", "slug": "KAN",
        "columns": [
            {"id": "c1", "slug": "to-do", "name": "To Do", "isFinal": false, "tasks": tasks(todo)},
            {"id": "c2", "slug": "done", "name": "Done", "isFinal": true, "tasks": tasks(done)}
        ],
        "plannedTasks": tasks(planned),
        "archivedTasks": tasks(archived)
    }))
    .unwrap_or_else(|e| panic!("fixture: {e}"))
}

fn ids(board: &LoadedBoard) -> Vec<&str> {
    board.entries.iter().map(|e| e.task.id.as_str()).collect()
}

#[test]
fn done_and_archived_tasks_are_hidden_by_default() {
    let page = data(&["a"], &["b"], &["c"], &["d"]);
    let mut board = LoadedBoard::empty(&page, 4);
    board.absorb(page, false);
    assert_eq!(ids(&board), ["a", "c"]);
    assert_eq!(board.entries[1].bucket, Bucket::Planned);
}

#[test]
fn include_done_keeps_every_bucket_across_pages() {
    let first = data(&["a"], &["b"], &[], &[]);
    let second = data(&[], &["c"], &["d"], &["e"]);
    let mut board = LoadedBoard::empty(&first, 5);
    board.absorb(first, true);
    board.absorb(second, true);
    assert_eq!(ids(&board), ["a", "b", "c", "d", "e"]);
    assert!(board.entries[1].bucket.is_done());
    assert!(board.entries[4].bucket.is_done());
    assert_eq!(board.project_slug, "KAN");
}

#[test]
fn related_pages_append_labels_to_loaded_tasks_only() {
    let page = data(&["a", "b"], &[], &[], &[]);
    let mut board = LoadedBoard::empty(&page, 2);
    board.absorb(page, false);

    let mut extra = data(&[], &[], &[], &[]);
    let label = |id: &str| json!([{"id": id, "name": id, "color": "#000"}]);
    extra.columns[0].tasks = vec![
        serde_json::from_value(task("a", label("l1"))).unwrap_or_else(|e| panic!("{e}")),
        serde_json::from_value(task("zzz", label("l2"))).unwrap_or_else(|e| panic!("{e}")),
    ];
    board.merge_labels(extra);

    assert_eq!(board.entries[0].task.labels.len(), 1);
    assert_eq!(board.entries[0].task.labels[0].id, "l1");
    assert!(board.entries[1].task.labels.is_empty());
    assert_eq!(board.entries.len(), 2);
}
