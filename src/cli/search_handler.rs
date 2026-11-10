use serde::Serialize;

use crate::api::types::search::{SearchHit, SearchResponse};
use crate::auth::context::{self, ResolvedContext};
use crate::cli::args::SearchArgs;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchQuery {
    q: String,
    workspace_id: String,
    #[serde(rename = "type")]
    kind: &'static str,
    limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
}

pub async fn run(args: SearchArgs, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let query = SearchQuery {
        q: args.query,
        workspace_id: context::require_workspace(None, ctx)?,
        kind: args.r#type.as_api_str(),
        limit: args.limit,
        project_id: args.project_id,
    };
    let response: SearchResponse = ctx
        .client()?
        .get_query("/search", &query)
        .await
        .with_action("Search failed")?;

    if response.results.is_empty() {
        output::warn("No results found");
        return Ok(());
    }
    for (title, hits) in response.grouped() {
        output::header(title);
        for hit in hits {
            println!("{}", describe(hit));
        }
    }
    eprintln!(
        "\n  {} shown, {} total",
        response.results.len(),
        response.total_count
    );
    Ok(())
}

fn describe(hit: &SearchHit) -> String {
    match hit.kind.as_str() {
        "task" => {
            let ticket = match (&hit.project_slug, hit.task_number) {
                (Some(slug), Some(number)) => format!("{slug}-{number}"),
                _ => "-".to_owned(),
            };
            let status = hit.status.as_deref().unwrap_or("");
            format!("{ticket:<12} {}  {status}  {}", hit.title, hit.id)
        }
        "comment" | "activity" => {
            let text = hit.content.as_deref().unwrap_or(&hit.title);
            format!("{}  {}", output::truncate(text, 80), hit.id)
        }
        _ => format!("{}  {}", hit.title, hit.id),
    }
}
