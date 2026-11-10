use chrono::{DateTime, FixedOffset};
use clap::Subcommand;

use crate::cli::duration::{LogDay, WorkDuration};

/// Time tracking subcommands. Every `<task>` argument accepts a ticket ID such as `KAN-12` or a task ID.
#[derive(Subcommand, Clone)]
pub enum TimeCommand {
    #[command(about = "Start a timer on a task")]
    Start {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(short = 'm', long = "note", help = "What you are working on")]
        note: Option<String>,
    },

    #[command(about = "Stop your running timer on a task")]
    Stop {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(
        about = "Log finished work on a task",
        after_help = "Durations: 1h30m, 45m, 2h, 1.5h, or bare minutes such as 90.\n\
                      Without --date the entry ends now; with --date it starts at 09:00 local time that day."
    )]
    Log {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
        #[arg(help = "Time spent, e.g. 1h30m, 45m, 1.5h, or 90 (minutes)")]
        duration: WorkDuration,
        #[arg(long, help = "Day of the work: YYYY-MM-DD, today, or yesterday")]
        date: Option<LogDay>,
        #[arg(short = 'm', long = "note", help = "What the time was spent on")]
        note: Option<String>,
    },

    #[command(visible_alias = "ls", about = "List time entries on a task")]
    List {
        #[arg(help = "Ticket ID (e.g. KAN-12) or task ID")]
        task: String,
    },

    #[command(about = "Change a time entry's start, duration, or note")]
    Edit {
        #[arg(help = "Time entry ID")]
        id: String,
        #[arg(long, help = "New duration, e.g. 1h30m; closes a running entry")]
        duration: Option<WorkDuration>,
        #[arg(
            long,
            help = "New start time (RFC 3339, e.g. 2026-01-31T09:00:00Z); keeps the duration"
        )]
        start: Option<DateTime<FixedOffset>>,
        #[arg(short = 'm', long = "note", help = "New note")]
        note: Option<String>,
    },
}
