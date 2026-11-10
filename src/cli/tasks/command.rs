use clap::Subcommand;

use crate::cli::tasks::activity::ActivityArgs;
use crate::cli::tasks::bulk::BulkArgs;
use crate::cli::tasks::comment::TaskCommentCommand;
use crate::cli::tasks::crud::{CreateArgs, DeleteArgs, TaskArg, UpdateArgs};
use crate::cli::tasks::duplicate::DuplicateArgs;
use crate::cli::tasks::field::FieldArgs;
use crate::cli::tasks::label::TaskLabelCommand;
use crate::cli::tasks::link::TaskLinkCommand;
use crate::cli::tasks::list::ListArgs;
use crate::cli::tasks::mine::MineArgs;
use crate::cli::tasks::quick::{AssignArgs, MoveArgs, PriorityArgs, StatusArgs};
use crate::cli::tasks::relation::TaskRelationCommand;
use crate::cli::tasks::repeat::RepeatArgs;
use crate::cli::tasks::transfer::{ExportArgs, ImportArgs};

/// Task subcommands. Every `<task>` argument accepts a ticket ID such as `KAN-12` or a task ID.
#[derive(Subcommand, Clone)]
pub enum TaskCommand {
    #[command(
        visible_alias = "ls",
        about = "List tasks of a project, following every page"
    )]
    List(ListArgs),

    #[command(about = "Show a task")]
    Get(TaskArg),

    #[command(about = "Create a task")]
    Create(CreateArgs),

    #[command(about = "Update task fields")]
    Update(UpdateArgs),

    #[command(visible_alias = "rm", about = "Delete a task")]
    Delete(DeleteArgs),

    #[command(about = "Move a task to another column by its slug")]
    Status(StatusArgs),

    #[command(about = "Set a task's priority")]
    Priority(PriorityArgs),

    #[command(about = "Assign or unassign a task")]
    Assign(AssignArgs),

    #[command(name = "move", about = "Move a task to another project")]
    MoveTask(MoveArgs),

    #[command(about = "Manage comments on a task")]
    Comment {
        #[command(subcommand)]
        command: TaskCommentCommand,
    },

    #[command(name = "label", about = "Manage labels on a task")]
    TaskLabel {
        #[command(subcommand)]
        command: TaskLabelCommand,
    },

    #[command(about = "Manage relations between tasks")]
    Relation {
        #[command(subcommand)]
        command: TaskRelationCommand,
    },

    #[command(about = "Manage external links and pull requests on a task")]
    Link {
        #[command(subcommand)]
        command: TaskLinkCommand,
    },

    #[command(about = "Show a task's custom field values, or set one")]
    Field(FieldArgs),

    #[command(about = "Show a task's activity feed, newest first")]
    Activity(ActivityArgs),

    #[command(about = "Make a task repeat when completed, or stop it repeating")]
    Repeat(RepeatArgs),

    #[command(about = "Copy a task into the same project and column")]
    Duplicate(DuplicateArgs),

    #[command(about = "Apply one change to many tasks at once")]
    Bulk(BulkArgs),

    #[command(about = "List open tasks assigned to you across a workspace")]
    Mine(MineArgs),

    #[command(about = "Export a project's tasks as JSON")]
    Export(ExportArgs),

    #[command(about = "Import tasks into a project from a JSON file")]
    Import(ImportArgs),
}
