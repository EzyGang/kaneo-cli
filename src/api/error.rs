use std::fmt;

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// A failed request to the Kaneo API.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("cannot reach {url}: {source}")]
    Transport {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("{status}: {body}")]
    Status { status: StatusCode, body: ErrorBody },

    #[error("unexpected response from {url}: {source}")]
    Decode {
        url: String,
        #[source]
        source: serde_json::Error,
    },
}

/// The JSON error body that every Kaneo API error carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub issues: Vec<Issue>,
    #[serde(default)]
    pub missing_permissions: Vec<String>,
}

/// One failed validation rule of a request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Issue {
    pub path: String,
    pub message: String,
}

impl ApiError {
    /// Returns the stable error code of an HTTP error, such as `NOT_FOUND`.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Status { body, .. } => Some(body.code.as_str()),
            Self::Transport { .. } | Self::Decode { .. } => None,
        }
    }

    /// Returns the HTTP status of an HTTP error.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Status { status, .. } => Some(*status),
            Self::Transport { .. } | Self::Decode { .. } => None,
        }
    }
}

impl ErrorBody {
    /// Parses an error response, falling back to the raw text for non-JSON bodies.
    #[must_use]
    pub fn parse(status: StatusCode, text: &str) -> Self {
        match serde_json::from_str::<Self>(text) {
            Ok(body) if !body.message.is_empty() || !body.code.is_empty() => body,
            _ => Self {
                message: text.trim().chars().take(300).collect(),
                code: default_code(status),
                ..Self::default()
            },
        }
    }
}

impl fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if !self.code.is_empty() {
            write!(f, " ({})", self.code)?;
        }
        for issue in &self.issues {
            write!(f, "\n    - {}: {}", issue.path, issue.message)?;
        }
        if !self.missing_permissions.is_empty() {
            write!(
                f,
                "\n    missing permissions: {}",
                self.missing_permissions.join(", ")
            )?;
        }
        Ok(())
    }
}

fn default_code(status: StatusCode) -> String {
    status
        .canonical_reason()
        .unwrap_or("HTTP_ERROR")
        .to_uppercase()
        .replace(' ', "_")
}
