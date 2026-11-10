use chrono::{DateTime, FixedOffset, Local, SecondsFormat, TimeDelta, Utc};

use crate::cli::duration::LogDay;
use crate::errors::KaneoError;

/// Local hour at which entries logged with `--date` start.
const LOG_DAY_START_HOUR: u32 = 9;

/// Returns the UTC instant at which an entry logged for `day` starts.
pub fn day_start(day: LogDay) -> Result<DateTime<Utc>, KaneoError> {
    let invalid = || KaneoError::Usage("--date does not resolve to a valid local time".to_owned());
    day.resolve(Local::now().date_naive())
        .and_then(|date| date.and_hms_opt(LOG_DAY_START_HOUR, 0, 0))
        .and_then(|at| at.and_local_timezone(Local).earliest())
        .map(|at| at.with_timezone(&Utc))
        .ok_or_else(invalid)
}

pub fn shift(at: DateTime<Utc>, by: TimeDelta) -> Result<DateTime<Utc>, KaneoError> {
    at.checked_add_signed(by)
        .ok_or_else(|| KaneoError::Usage("The resulting time is out of range".to_owned()))
}

pub fn parse_timestamp(value: &str) -> Result<DateTime<FixedOffset>, KaneoError> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|e| KaneoError::Usage(format!("Unexpected timestamp '{value}' from the API: {e}")))
}

/// Formats an instant as the RFC 3339 UTC string the API expects.
#[must_use]
pub fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Secs, true)
}
