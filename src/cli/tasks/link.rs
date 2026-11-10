use clap::Subcommand;

use crate::api::types::external_link::{
    CreateExternalLinkBody, ExternalLink, LinkResourceType, PullRequestStatus,
    UpdateExternalLinkBody,
};
use crate::cli::tasks::env::TaskEnv;
use crate::errors::{ApiResultExt, KaneoError};
use crate::output;

#[derive(Subcommand, Clone)]
pub enum TaskLinkCommand {
    #[command(
        visible_alias = "ls",
        about = "List links and pull requests attached to a task"
    )]
    List {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(about = "Attach a URL or pull request link to a task")]
    Add {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "HTTP or HTTPS URL")]
        url: String,
        #[arg(long, help = "Display title (up to 200 characters)")]
        title: Option<String>,
        #[arg(
            long,
            help = "Show the link as a pull request (status defaults to open)"
        )]
        pr: bool,
        #[arg(long, value_enum, help = "Pull request status; implies --pr")]
        state: Option<PullRequestStatus>,
    },

    #[command(about = "Change the title or pull request status of a manual link")]
    Update {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "Link ID")]
        id: String,
        #[arg(long, help = "New display title")]
        title: Option<String>,
        #[arg(
            long,
            value_enum,
            help = "New pull request status (pull request links only)"
        )]
        state: Option<PullRequestStatus>,
    },

    #[command(visible_alias = "rm", about = "Remove a manual link from a task")]
    Remove {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "Link ID")]
        id: String,
        #[arg(long, help = "Confirm the removal")]
        force: bool,
    },
}

pub async fn run(cmd: TaskLinkCommand, env: &TaskEnv<'_>) -> Result<(), KaneoError> {
    match cmd {
        TaskLinkCommand::List { task } => {
            let task_id = env.tasks.id(&task).await?;
            let links: Vec<ExternalLink> = env
                .client
                .get(&format!("/external-link/task/{task_id}"))
                .await
                .with_action("Failed to list links")?;
            if links.is_empty() {
                output::warn("No links");
                return Ok(());
            }
            print_table(&links);
        }

        TaskLinkCommand::Add {
            task,
            url,
            title,
            pr,
            state,
        } => {
            let task_id = env.tasks.id(&task).await?;
            let status = state.or(pr.then_some(PullRequestStatus::Open));
            let body = CreateExternalLinkBody {
                url,
                title,
                resource_type: match status {
                    Some(_) => LinkResourceType::PullRequest,
                    None => LinkResourceType::Url,
                },
                metadata: status.map(PullRequestStatus::metadata),
            };
            let link: ExternalLink = env
                .client
                .post(&format!("/external-link/task/{task_id}"), &body)
                .await
                .with_action("Failed to add link")?;
            output::success(&format!("Link added ({})", link.id));
        }

        TaskLinkCommand::Update {
            task,
            id,
            title,
            state,
        } => {
            if title.is_none() && state.is_none() {
                return Err(KaneoError::Usage(
                    "Nothing to change: pass --title or --state".to_owned(),
                ));
            }
            let task_id = env.tasks.id(&task).await?;
            let body = UpdateExternalLinkBody {
                title,
                metadata: state.map(PullRequestStatus::metadata),
            };
            let _: ExternalLink = env
                .client
                .patch(&link_path(&task_id, &id), &body)
                .await
                .with_action("Failed to update link")?;
            output::success("Link updated");
        }

        TaskLinkCommand::Remove { task, id, force } => {
            if !force {
                output::force_required();
                return Ok(());
            }
            let task_id = env.tasks.id(&task).await?;
            let _: serde_json::Value = env
                .client
                .delete(&link_path(&task_id, &id))
                .await
                .with_action("Failed to remove link")?;
            output::success("Link removed");
        }
    }
    Ok(())
}

fn print_table(links: &[ExternalLink]) {
    println!(
        "{:<22} {:<8} {:<40} {:<50} ID",
        "Type", "Status", "Title", "URL"
    );
    for link in links {
        let kind = match &link.integration {
            Some(integration) => format!("{}:{}", integration.kind, link.resource_type),
            None => link.resource_type.clone(),
        };
        let status = link.pr_status().unwrap_or("-");
        let title = output::truncate(link.title.as_deref().unwrap_or("-"), 40);
        println!(
            "{kind:<22} {status:<8} {title:<40} {:<50} {}",
            link.url, link.id
        );
    }
}

fn link_path(task_id: &str, id: &str) -> String {
    format!("/external-link/task/{task_id}/{id}")
}
