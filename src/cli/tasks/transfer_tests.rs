use serde_json::{Value, json};

use crate::cli::tasks::transfer::import_tasks;
use crate::errors::KaneoError;

fn tasks(document: Value) -> Vec<Value> {
    import_tasks(document).unwrap_or_else(|e| panic!("import_tasks failed: {e}"))
}

fn assert_usage(result: Result<Vec<Value>, KaneoError>) {
    match result {
        Err(KaneoError::Usage(_)) => {}
        other => panic!("expected a usage error, got {other:?}"),
    }
}

#[test]
fn export_documents_drop_fields_the_import_rejects() {
    let document = json!({
        "project": {"name": "Kaneo"},
        "tasks": [{
            "title": "Ship", "description": "", "status": "to-do", "priority": "high",
            "dueDate": null, "startDate": null, "userId": "u1",
            "labels": [{"name": "bug", "color": "#f00"}]
        }]
    });
    let expected = json!({
        "title": "Ship", "description": "", "status": "to-do", "priority": "high",
        "dueDate": null, "startDate": null, "userId": "u1"
    });
    assert_eq!(tasks(document), vec![expected]);
}

#[test]
fn bare_task_lists_are_accepted() {
    let document = json!({"tasks": [{"title": "A", "status": "planned"}]});
    assert_eq!(
        tasks(document),
        vec![json!({"title": "A", "status": "planned"})]
    );
}

#[test]
fn documents_without_a_tasks_array_are_rejected() {
    assert_usage(import_tasks(json!([])));
    assert_usage(import_tasks(json!({"tasks": {}})));
}

#[test]
fn tasks_need_a_title_and_status() {
    assert_usage(import_tasks(json!({"tasks": [{"title": "A"}]})));
    assert_usage(import_tasks(json!({"tasks": ["A"]})));
}
