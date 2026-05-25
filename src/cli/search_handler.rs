use crate::api::client::ApiClient;
use crate::auth::context;
use crate::cli::SearchArgs;
use crate::output;

pub async fn run(
    args: SearchArgs,
    ctx: &context::ResolvedContext,
) -> Result<(), crate::errors::KaneoError> {
    let ws = context::require_workspace(None, ctx)?;
    let client =
        ApiClient::new(&ctx.instance, &ctx.api_key).map_err(crate::errors::config_error)?;

    #[derive(serde::Serialize)]
    struct QueryParams {
        q: String,
        #[serde(rename = "workspaceId")]
        workspace_id: String,
        #[serde(rename = "type")]
        r#type: String,
        limit: String,
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        project_id: Option<String>,
    }

    let query = QueryParams {
        q: args.query,
        workspace_id: ws,
        r#type: args.r#type.as_api_str().to_owned(),
        limit: args.limit.to_string(),
        project_id: args.project_id,
    };

    let results: serde_json::Value = client
        .get_query("/search", &query)
        .await
        .map_err(|e| crate::errors::api_error("Search failed".to_owned(), e))?;

    let mut found = false;

    if let Some(tasks) = results.get("tasks").and_then(|v| v.as_array())
        && !tasks.is_empty()
    {
        found = true;
        output::header("Tasks");
        for t in tasks {
            let title = t.get("title").and_then(|v| v.as_str()).unwrap_or("?");
            let id = t.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let number = t.get("number").and_then(|v| v.as_i64()).unwrap_or(0);
            let status = t.get("status").and_then(|v| v.as_str()).unwrap_or("");
            println!("#{number}  {title}  {status}  {id}");
        }
    }

    if let Some(projects) = results.get("projects").and_then(|v| v.as_array())
        && !projects.is_empty()
    {
        found = true;
        output::header("Projects");
        for p in projects {
            let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let id = p.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("{name}  {id}");
        }
    }

    if let Some(workspaces) = results.get("workspaces").and_then(|v| v.as_array())
        && !workspaces.is_empty()
    {
        found = true;
        output::header("Workspaces");
        for w in workspaces {
            let name = w.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let id = w.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("{name}  {id}");
        }
    }

    if let Some(comments) = results.get("comments").and_then(|v| v.as_array())
        && !comments.is_empty()
    {
        found = true;
        output::header("Comments");
        for c in comments {
            let content = c.get("content").and_then(|v| v.as_str()).unwrap_or("?");
            let id = c.get("id").and_then(|v| v.as_str()).unwrap_or("");
            println!("{content}  {id}");
        }
    }

    if !found {
        output::warn("No results found");
    }

    eprintln!();
    Ok(())
}
