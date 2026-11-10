use std::io::Read;
use std::path::PathBuf;

use clap::Args;
use serde_json::{Map, Value, json};

use crate::api::types::task_ops::TaskImportResult;
use crate::auth::context;
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

/// Task fields accepted by `POST /task/import/{projectId}`.
const IMPORT_FIELDS: [&str; 7] = [
    "title",
    "description",
    "status",
    "priority",
    "startDate",
    "dueDate",
    "userId",
];

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ExportArgs {
    #[arg(long, help = "Project ID")]
    pub project_id: Option<String>,
    #[arg(short, long, help = "Write the export to this file instead of stdout")]
    pub output: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq, Eq)]
pub struct ImportArgs {
    #[arg(help = "JSON file from `task export` or `{\"tasks\": [...]}`, or `-` for stdin")]
    pub file: String,
    #[arg(long, help = "Project ID")]
    pub project_id: Option<String>,
    #[arg(
        long,
        help = "Import the tasks; without it, only report how many would be imported"
    )]
    pub force: bool,
}

pub async fn export(args: ExportArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let pid = context::require_project(args.project_id.as_deref(), env.ctx)?;
    let document: Value = env
        .client
        .get(&format!("/task/export/{pid}"))
        .await
        .with_action("Failed to export tasks")?;
    let pretty = serde_json::to_string_pretty(&document)
        .map_err(|e| KaneoError::Usage(format!("Failed to format the export: {e}")))?;
    match args.output {
        Some(path) => {
            std::fs::write(&path, format!("{pretty}\n"))?;
            output::success(&format!("Exported tasks to {}", path.display()));
        }
        None => println!("{pretty}"),
    }
    Ok(())
}

pub async fn import(args: ImportArgs, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    let pid = context::require_project(args.project_id.as_deref(), env.ctx)?;
    let text = read_input(&args.file)?;
    let document: Value = serde_json::from_str(&text)
        .map_err(|e| KaneoError::Usage(format!("{} is not valid JSON: {e}", args.file)))?;
    let tasks = import_tasks(document)?;
    if !args.force {
        output::warn(&format!(
            "Would import {} tasks; re-run with `--force` to import them",
            tasks.len()
        ));
        return Ok(());
    }
    let result: TaskImportResult = env
        .client
        .post(&format!("/task/import/{pid}"), &json!({ "tasks": tasks }))
        .await
        .with_action("Failed to import tasks")?;
    let summary = result.results;
    for task in &summary.tasks {
        println!("{}", describe_outcome(task));
    }
    output::success(&format!(
        "Imported {} of {} tasks ({} failed)",
        summary.successful, summary.total, summary.failed
    ));
    Ok(())
}

/// Extracts the import payload from an export document or a bare `{tasks: [...]}`,
/// keeping only the task fields the import endpoint accepts.
pub fn import_tasks(document: Value) -> Result<Vec<Value>, KaneoError> {
    let tasks = match document {
        Value::Object(mut root) => root.remove("tasks"),
        _ => None,
    };
    let Some(Value::Array(tasks)) = tasks else {
        return Err(KaneoError::Usage(
            "Expected a JSON object with a `tasks` array".to_owned(),
        ));
    };
    tasks
        .into_iter()
        .enumerate()
        .map(|(index, task)| import_task(index, task))
        .collect()
}

fn import_task(index: usize, task: Value) -> Result<Value, KaneoError> {
    let Value::Object(mut fields) = task else {
        return Err(KaneoError::Usage(format!(
            "Task #{} is not an object",
            index + 1
        )));
    };
    for required in ["title", "status"] {
        match fields.get(required) {
            Some(Value::String(_)) => {}
            _ => {
                return Err(KaneoError::Usage(format!(
                    "Task #{} is missing a `{required}` string",
                    index + 1
                )));
            }
        }
    }
    let kept: Map<String, Value> = IMPORT_FIELDS
        .iter()
        .filter_map(|key| fields.remove(*key).map(|value| ((*key).to_owned(), value)))
        .collect();
    Ok(Value::Object(kept))
}

/// Formats one per-task import outcome, whose shape the API only guarantees has `success`.
fn describe_outcome(outcome: &Value) -> String {
    let ok = outcome
        .get("success")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let mark = if ok { "ok    " } else { "failed" };
    let title = outcome
        .get("title")
        .or_else(|| outcome.get("task").and_then(|t| t.get("title")))
        .and_then(Value::as_str)
        .unwrap_or("-");
    let detail = ["error", "message", "warning"]
        .iter()
        .find_map(|key| outcome.get(*key).and_then(Value::as_str));
    match detail {
        Some(detail) => format!("{mark}  {title}  ({detail})"),
        None => format!("{mark}  {title}"),
    }
}

fn read_input(file: &str) -> Result<String, KaneoError> {
    if file != "-" {
        return Ok(std::fs::read_to_string(file)?);
    }
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text)?;
    Ok(text)
}
