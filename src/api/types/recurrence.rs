use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// How often a repeating task comes back after it is completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// The repeat rule of a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recurrence {
    pub frequency: Frequency,
    pub interval: u32,
    /// Weekdays for weekly rules, where `0` is Sunday.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<u8>>,
    pub time_zone: String,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Daily => "day",
            Self::Weekly => "week",
            Self::Monthly => "month",
            Self::Yearly => "year",
        };
        f.write_str(name)
    }
}

impl fmt::Display for Recurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.interval {
            1 => write!(f, "every {}", self.frequency)?,
            n => write!(f, "every {n} {}s", self.frequency)?,
        }
        if let Some(days) = self.weekdays.as_deref().filter(|d| !d.is_empty()) {
            const NAMES: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
            let names: Vec<&str> = days
                .iter()
                .filter_map(|d| NAMES.get(usize::from(*d)).copied())
                .collect();
            write!(f, " on {}", names.join(","))?;
        }
        write!(f, " ({})", self.time_zone)
    }
}
