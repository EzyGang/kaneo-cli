use clap::ValueEnum;
use serde::Serialize;

/// A task priority accepted by the Kaneo API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Priority {
    NoPriority,
    #[default]
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    /// Returns the value the API uses for this priority.
    #[must_use]
    pub fn as_api_str(self) -> &'static str {
        match self {
            Self::NoPriority => "no-priority",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Urgent => "urgent",
        }
    }

    /// Returns a short column label for a priority string returned by the API.
    #[must_use]
    pub fn short_label(priority: &str) -> &'static str {
        match priority {
            "urgent" => "urgent",
            "high" => "high",
            "medium" => "med",
            "low" => "low",
            _ => "none",
        }
    }
}
