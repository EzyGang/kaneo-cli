use clap::Subcommand;

use crate::api::types::workflow::WorkflowEvent;

#[derive(Subcommand, Clone)]
pub enum WorkflowCommand {
    #[command(visible_alias = "ls", about = "List workflow rules of a project")]
    List {
        #[arg(long)]
        project_id: Option<String>,
    },

    #[command(about = "Create or replace the rule for an integration event")]
    Set {
        #[arg(help = "Integration that emits the event, e.g. github")]
        integration: String,
        #[arg(value_enum, help = "Event that triggers the move")]
        event: WorkflowEvent,
        #[arg(help = "Target column ID or slug")]
        column: String,
        #[arg(long)]
        project_id: Option<String>,
    },

    #[command(visible_alias = "rm", about = "Delete a workflow rule")]
    Delete {
        #[arg(help = "Workflow rule ID")]
        id: String,
        #[arg(long)]
        force: bool,
    },
}
