use crate::api::client::ApiClient;
use crate::api::types::{
    BoardResponse, Comment, CreateTaskBody, CreateTaskRelationBody, Label, MoveTaskResponse, Task,
    TaskRelation,
};
use crate::auth::context;
use crate::cli::tasks::{TaskCommand, TaskCommentCommand, TaskLabelCommand, TaskRelationCommand};
use crate::output;

pub async fn run(
    cmd: TaskCommand,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

    match cmd {
        TaskCommand::List {
            project_id,
            status,
            priority,
            sort,
            limit,
        } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let mut path = format!("/task/tasks/{pid}?limit={}", limit.unwrap_or(20));
            if let Some(s) = &status {
                path.push_str(&format!("&status={s}"));
            }
            if let Some(p) = &priority {
                path.push_str(&format!("&priority={p}"));
            }
            if let Some(s) = &sort {
                path.push_str(&format!("&sort={s}"));
            }
            let board: BoardResponse = client
                .get(&path)
                .await
                .map_err(|e| crate::errors::api_error("failed to list tasks".to_owned(), e))?;

            let slug = &board.data.slug;
            let mut tasks: Vec<&Task> = Vec::new();
            for col in &board.data.columns {
                tasks.extend(&col.tasks);
            }
            tasks.extend(&board.data.planned_tasks);
            tasks.extend(&board.data.archived_tasks);

            if tasks.is_empty() {
                output::warn("No tasks found");
                return Ok(());
            }

            print_task_table(slug, &tasks);
            eprintln!("\n  {} tasks", tasks.len());
        }

        TaskCommand::Get { id } => {
            let task: Task = client
                .get(&format!("/task/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Task", &id, e))?;

            let num = task.number.map(|n| format!("#{n}")).unwrap_or_default();
            let prio = priority_label(&task.priority);
            let assignee = task.assignee_name.as_deref().unwrap_or("-");
            let due = task.due_date.as_deref().unwrap_or("-");
            println!(
                "Ref        Number    Title                                      Priority  Status      Assignee  Due          ID"
            );
            println!(
                "{:<10} {:<9} {:<42} {:<9} {:<10} {:<10} {:<12} {}",
                num, num, task.title, prio, task.status, assignee, due, task.id
            );
            if let Some(desc) = &task.description
                && !desc.is_empty()
            {
                println!();
                println!("Description:");
                println!("{desc}");
            }
        }

        TaskCommand::Create {
            title,
            project_id,
            description,
            priority,
            status,
            due_date,
            start_date,
            assignee,
        } => {
            let pid = context::require_project(project_id.as_deref(), ctx)?;
            let body = CreateTaskBody {
                title,
                description: description.unwrap_or_default(),
                priority: priority.unwrap_or_else(|| "low".to_owned()),
                status: status.unwrap_or_else(|| "planned".to_owned()),
                due_date,
                start_date,
                user_id: assignee,
            };
            let task: Task = client
                .post(&format!("/task/{pid}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to create task".to_owned(), e))?;

            let num = task.number.map(|n| format!("#{n}")).unwrap_or_default();
            output::success(&format!(
                "Created task {num} '{}' ({})",
                task.title, task.id
            ));
        }

        TaskCommand::Update {
            id,
            title,
            description,
            priority,
            status,
            due_date,
            start_date,
            assignee,
        } => {
            let current: Task = client
                .get(&format!("/task/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Task", &id, e))?;

            #[derive(serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            struct UpdateBody {
                title: String,
                description: String,
                priority: String,
                status: String,
                project_id: String,
                position: f64,
                #[serde(skip_serializing_if = "Option::is_none")]
                due_date: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                start_date: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                user_id: Option<String>,
            }

            let body = UpdateBody {
                title: title.unwrap_or(current.title),
                description: description.unwrap_or(current.description.unwrap_or_default()),
                priority: priority.unwrap_or(current.priority),
                status: status.unwrap_or(current.status),
                project_id: current.project_id,
                position: current.position.unwrap_or(0.0),
                due_date: due_date.or(current.due_date),
                start_date: start_date.or(current.start_date),
                user_id: assignee.or(current.user_id),
            };

            let task: Task = client
                .put(&format!("/task/{id}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to update task".to_owned(), e))?;

            output::success(&format!("Updated task '{}'", task.title));
        }

        TaskCommand::Delete { id, force } => {
            if !force {
                eprintln!(
                    "  {} Re-run with `--force` to confirm deletion",
                    output::dim("note:")
                );
                return Ok(());
            }
            let task: Task = client
                .delete(&format!("/task/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Task", &id, e))?;

            output::success(&format!("Deleted task '{}'", task.title));
        }

        TaskCommand::Status { id, status } => {
            #[derive(serde::Serialize)]
            struct StatusBody {
                status: String,
            }
            let task: Task = client
                .put(&format!("/task/status/{id}"), &StatusBody { status })
                .await
                .map_err(|e| crate::errors::not_found("Task", &id, e))?;

            output::success(&format!("Task '{}' status -> {}", task.title, task.status));
        }

        TaskCommand::Priority { id, priority } => {
            #[derive(serde::Serialize)]
            struct PriorityBody {
                priority: String,
            }
            let task: Task = client
                .put(&format!("/task/priority/{id}"), &PriorityBody { priority })
                .await
                .map_err(|e| crate::errors::not_found("Task", &id, e))?;

            output::success(&format!(
                "Task '{}' priority -> {}",
                task.title,
                priority_label(&task.priority)
            ));
        }

        TaskCommand::Assign { id, user_id } => {
            #[derive(serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            struct AssignBody {
                user_id: String,
            }
            let body = AssignBody {
                user_id: user_id.unwrap_or_default(),
            };
            let task: Task = client
                .put(&format!("/task/assignee/{id}"), &body)
                .await
                .map_err(|e| crate::errors::api_error("failed to assign task".to_owned(), e))?;

            match &task.assignee_name {
                Some(name) => output::success(&format!("Task '{}' assigned to {name}", task.title)),
                None => output::success(&format!("Task '{}' unassigned", task.title)),
            }
        }

        TaskCommand::MoveTask { id, project_id } => {
            #[derive(serde::Serialize)]
            struct MoveBody {
                #[serde(rename = "destinationProjectId")]
                project_id: String,
            }
            let result: MoveTaskResponse = client
                .put(&format!("/task/move/{id}"), &MoveBody { project_id })
                .await
                .map_err(|e| crate::errors::api_error("failed to move task".to_owned(), e))?;

            output::success(&format!(
                "Task '{}' moved to project {}",
                result.task.title, result.destination_project_id
            ));
        }

        TaskCommand::Comment { command } => run_task_comment(command, &client).await?,
        TaskCommand::TaskLabel { command } => run_task_label(command, &client).await?,
        TaskCommand::Relation { command } => run_task_relation(command, &client).await?,
    }

    Ok(())
}

fn print_task_table(slug: &str, tasks: &[&Task]) {
    println!("Ref        Title                                      Priority  Status     ID");
    for t in tasks {
        let num = t.number.unwrap_or(0);
        let ref_id = format!("{slug}-{num}");
        let prio = priority_label(&t.priority);
        let status = &t.status;
        let title = &t.title;
        println!(
            "{:<10} {:<42} {:<9} {:<10} {}",
            ref_id, title, prio, status, t.id
        );
    }
}

async fn run_task_comment(
    cmd: TaskCommentCommand,
    client: &ApiClient,
) -> Result<(), crate::errors::KaneoError> {
    match cmd {
        TaskCommentCommand::List { task_id } => {
            let comments: Vec<Comment> = client
                .get(&format!("/comment/{task_id}"))
                .await
                .map_err(|e| crate::errors::api_error("failed to list comments".to_owned(), e))?;

            if comments.is_empty() {
                output::warn("No comments");
                return Ok(());
            }
            for c in &comments {
                let author = c
                    .user
                    .as_ref()
                    .map(|u| u.name.as_str())
                    .unwrap_or("unknown");
                let content = c.content.as_deref().unwrap_or("");
                println!("{}  {}  {}", c.id, author, content);
            }
        }

        TaskCommentCommand::Add { task_id, content } => {
            #[derive(serde::Serialize)]
            struct CommentBody {
                content: String,
            }
            let _comment: Comment = client
                .post(&format!("/comment/{task_id}"), &CommentBody { content })
                .await
                .map_err(|e| crate::errors::api_error("failed to add comment".to_owned(), e))?;

            output::success("Comment added");
        }

        TaskCommentCommand::Update { id, content } => {
            #[derive(serde::Serialize)]
            struct UpdateBody {
                content: String,
            }
            let _comment: Comment = client
                .put(&format!("/comment/{id}"), &UpdateBody { content })
                .await
                .map_err(|e| crate::errors::api_error("failed to update comment".to_owned(), e))?;

            output::success("Comment updated");
        }

        TaskCommentCommand::Delete { id } => {
            let _comment: Comment = client
                .delete(&format!("/comment/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Comment", &id, e))?;

            output::success("Comment deleted");
        }
    }

    Ok(())
}

async fn run_task_label(
    cmd: TaskLabelCommand,
    client: &ApiClient,
) -> Result<(), crate::errors::KaneoError> {
    match cmd {
        TaskLabelCommand::List { task_id } => {
            let labels: Vec<Label> = client
                .get(&format!("/label/task/{task_id}"))
                .await
                .map_err(|e| {
                    crate::errors::api_error("failed to list task labels".to_owned(), e)
                })?;

            if labels.is_empty() {
                output::warn("No labels on task");
                return Ok(());
            }
            for l in &labels {
                println!("{}  {}", l.name, l.id);
            }
        }

        TaskLabelCommand::Attach { task_id, label_id } => {
            #[derive(serde::Serialize)]
            struct AttachBody {
                #[serde(rename = "taskId")]
                task_id: String,
            }
            let _result: serde_json::Value = client
                .put(&format!("/label/{label_id}/task"), &AttachBody { task_id })
                .await
                .map_err(|e| crate::errors::api_error("failed to attach label".to_owned(), e))?;

            output::success("Label attached to task");
        }

        TaskLabelCommand::Detach { task_id, label_id } => {
            #[derive(serde::Serialize)]
            struct DetachBody {
                #[serde(rename = "taskId")]
                task_id: String,
            }
            let _result: serde_json::Value = client
                .delete_json(&format!("/label/{label_id}/task"), &DetachBody { task_id })
                .await
                .map_err(|e| crate::errors::api_error("failed to detach label".to_owned(), e))?;

            output::success("Label detached from task");
        }
    }

    Ok(())
}

async fn run_task_relation(
    cmd: TaskRelationCommand,
    client: &ApiClient,
) -> Result<(), crate::errors::KaneoError> {
    match cmd {
        TaskRelationCommand::List { task_id } => {
            let relations: Vec<TaskRelation> = client
                .get(&format!("/task-relation/{task_id}"))
                .await
                .map_err(|e| {
                    crate::errors::api_error("failed to list task relations".to_owned(), e)
                })?;

            if relations.is_empty() {
                output::warn("No relations found for this task");
                return Ok(());
            }
            for r in &relations {
                println!(
                    "{}  {} -> {} ({})",
                    r.id, r.source_task_id, r.target_task_id, r.relation_type
                );
            }
        }

        TaskRelationCommand::Create {
            source_task_id,
            target_task_id,
            relation_type,
        } => {
            let body = CreateTaskRelationBody {
                source_task_id,
                target_task_id,
                relation_type,
            };
            let relation: TaskRelation =
                client.post("/task-relation", &body).await.map_err(|e| {
                    crate::errors::api_error("failed to create task relation".to_owned(), e)
                })?;

            output::success(&format!(
                "Created {} relation between {} and {} ({})",
                relation.relation_type,
                relation.source_task_id,
                relation.target_task_id,
                relation.id
            ));
        }

        TaskRelationCommand::Delete { id } => {
            let relation: TaskRelation = client
                .delete(&format!("/task-relation/{id}"))
                .await
                .map_err(|e| crate::errors::not_found("Task relation", &id, e))?;

            output::success(&format!(
                "Deleted {} relation ({})",
                relation.relation_type, relation.id
            ));
        }
    }

    Ok(())
}

fn priority_label(priority: &str) -> &str {
    match priority {
        "urgent" => "urgent",
        "high" => "high",
        "medium" => "med",
        "low" => "low",
        _ => "none",
    }
}
