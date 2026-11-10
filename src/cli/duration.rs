use std::str::FromStr;

use chrono::{NaiveDate, TimeDelta};

/// A positive span of tracked time, written as `1h30m`, `45m`, `2h`, `1.5h`, or bare minutes such as `90`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkDuration(TimeDelta);

/// The day a logged entry belongs to: `today`, `yesterday`, or a `YYYY-MM-DD` date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogDay {
    Today,
    Yesterday,
    On(NaiveDate),
}

/// Units in the order they must appear, with their length in seconds.
const UNITS: [(char, u64); 2] = [('h', 3600), ('m', 60)];
const MAX_FRACTION_DIGITS: usize = 6;

impl WorkDuration {
    #[must_use]
    pub fn delta(self) -> TimeDelta {
        self.0
    }
}

impl FromStr for WorkDuration {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let compact = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase();
        let parsed = if compact.bytes().all(|b| b.is_ascii_digit()) {
            scaled(&compact, 60)
        } else {
            with_units(&compact)
        };
        let seconds = parsed.ok_or_else(|| {
            format!(
                "invalid duration '{input}': use forms like 1h30m, 45m, 2h, 1.5h or 90 (minutes)"
            )
        })?;
        if seconds == 0 {
            return Err("duration must be greater than zero".to_owned());
        }
        i64::try_from(seconds)
            .ok()
            .and_then(TimeDelta::try_seconds)
            .map(Self)
            .ok_or_else(|| format!("duration '{input}' is too long"))
    }
}

impl FromStr for LogDay {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_ascii_lowercase().as_str() {
            "today" => Ok(Self::Today),
            "yesterday" => Ok(Self::Yesterday),
            other => NaiveDate::parse_from_str(other, "%Y-%m-%d")
                .map(Self::On)
                .map_err(|_| format!("invalid date '{input}': use YYYY-MM-DD, today or yesterday")),
        }
    }
}

impl LogDay {
    /// Returns the calendar date relative to `today`.
    #[must_use]
    pub fn resolve(self, today: NaiveDate) -> Option<NaiveDate> {
        match self {
            Self::Today => Some(today),
            Self::Yesterday => today.pred_opt(),
            Self::On(date) => Some(date),
        }
    }
}

/// Formats whole seconds as `h:mm`, rounded to the nearest minute.
#[must_use]
pub fn hours_minutes(seconds: i64) -> String {
    let minutes = (seconds.max(0) + 30) / 60;
    format!("{}:{:02}", minutes / 60, minutes % 60)
}

/// Sums `<number><unit>` segments such as `1h30m`, with each unit at most once and hours first.
fn with_units(text: &str) -> Option<u64> {
    let mut total: u64 = 0;
    let mut rest = text;
    let mut allowed: &[(char, u64)] = &UNITS;
    while !rest.is_empty() {
        let end = rest.find(|c: char| !(c.is_ascii_digit() || c == '.'))?;
        let (number, tail) = rest.split_at(end);
        let unit = tail.chars().next()?;
        let index = allowed.iter().position(|(name, _)| *name == unit)?;
        let (_, unit_seconds) = allowed.get(index)?;
        total = total.checked_add(scaled(number, *unit_seconds)?)?;
        allowed = allowed.get(index + 1..)?;
        rest = tail.get(unit.len_utf8()..)?;
    }
    Some(total)
}

/// Converts a decimal count of units into whole seconds, rounding down.
fn scaled(number: &str, unit_seconds: u64) -> Option<u64> {
    let (whole, fraction) = number.split_once('.').unwrap_or((number, ""));
    let digits = |part: &str| part.bytes().all(|b| b.is_ascii_digit());
    if (whole.is_empty() && fraction.is_empty())
        || !digits(whole)
        || !digits(fraction)
        || fraction.len() > MAX_FRACTION_DIGITS
    {
        return None;
    }
    let value = |part: &str| {
        if part.is_empty() {
            return Some(0);
        }
        part.parse::<u64>().ok()
    };
    let scale = 10_u64.checked_pow(u32::try_from(fraction.len()).ok()?)?;
    let fractional = value(fraction)?.checked_mul(unit_seconds)? / scale;
    value(whole)?
        .checked_mul(unit_seconds)?
        .checked_add(fractional)
}
