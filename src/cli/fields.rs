use clap::Subcommand;

use crate::api::types::custom_field::FieldType;

#[derive(Subcommand, Clone)]
pub enum FieldCommand {
    #[command(visible_alias = "ls", about = "List custom fields of a project")]
    List {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long, help = "Also show hidden (retired) options")]
        all: bool,
    },

    #[command(about = "Create a custom field")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long = "type", value_enum)]
        field_type: FieldType,
        #[arg(long, help = "Every task must have a value")]
        required: bool,
        #[arg(long, help = "Default value (comma-separated for multiselect)")]
        default: Option<String>,
        #[arg(
            long = "option",
            help = "Option label for dropdown/multiselect (repeatable)"
        )]
        options: Vec<String>,
        #[arg(long)]
        project_id: Option<String>,
    },

    #[command(about = "Update a custom field")]
    Update {
        #[arg(help = "Custom field ID")]
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(
            long = "type",
            value_enum,
            help = "Only dropdown <-> multiselect is allowed"
        )]
        field_type: Option<FieldType>,
        #[arg(long)]
        required: Option<bool>,
        #[arg(long, conflicts_with = "clear_default")]
        default: Option<String>,
        #[arg(long)]
        clear_default: bool,
        #[arg(long = "option", help = "Full option list, in order (repeatable)")]
        options: Vec<String>,
        #[arg(
            long = "hide-option",
            help = "Option to retire (repeatable; full hidden list)"
        )]
        hide_options: Vec<String>,
    },

    #[command(visible_alias = "rm", about = "Delete a custom field")]
    Delete {
        #[arg(help = "Custom field ID")]
        id: String,
        #[arg(long)]
        force: bool,
    },

    #[command(about = "Set the display order of custom fields")]
    Reorder {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long, help = "Comma-separated field IDs in the new order")]
        order: String,
    },
}
