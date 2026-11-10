use chrono::{NaiveDate, TimeDelta};

use crate::cli::duration::{LogDay, WorkDuration, hours_minutes};

fn seconds(input: &str) -> i64 {
    input
        .parse::<WorkDuration>()
        .unwrap_or_else(|e| panic!("{input}: {e}"))
        .delta()
        .num_seconds()
}

fn date(text: &str) -> NaiveDate {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap_or_else(|e| panic!("{text}: {e}"))
}

#[test]
fn unit_forms_are_parsed() {
    assert_eq!(seconds("1h30m"), 5400);
    assert_eq!(seconds("45m"), 2700);
    assert_eq!(seconds("2h"), 7200);
    assert_eq!(seconds("1.5h"), 5400);
    assert_eq!(seconds("0.25h"), 900);
    assert_eq!(seconds("1H 15M"), 4500);
}

#[test]
fn bare_numbers_are_minutes() {
    assert_eq!(seconds("90"), 5400);
    assert_eq!(seconds(" 5 "), 300);
}

#[test]
fn zero_durations_are_rejected() {
    for input in ["0", "0h", "0m", "0h0m", "0.0h"] {
        let err = input.parse::<WorkDuration>().err();
        assert_eq!(
            err.as_deref(),
            Some("duration must be greater than zero"),
            "{input}"
        );
    }
}

#[test]
fn malformed_durations_are_rejected() {
    let inputs = [
        "", "-1h", "-30", "h", "m", "abc", "1x", "1h30", "30m1h", "1h1h", "1.2.3h", "1hm", "1.5",
        "1d", "1h-5m",
    ];
    for input in inputs {
        assert!(
            input.parse::<WorkDuration>().is_err(),
            "{input} should be rejected"
        );
    }
}

#[test]
fn oversized_durations_are_rejected() {
    assert!("99999999999999999999h".parse::<WorkDuration>().is_err());
    assert!(
        "5124095576030431h"
            .parse::<WorkDuration>()
            .is_err_and(|e| e.contains("too long"))
    );
}

#[test]
fn log_days_resolve_relative_to_today() {
    let today = date("2026-03-01");
    let day = |input: &str| {
        input
            .parse::<LogDay>()
            .unwrap_or_else(|e| panic!("{input}: {e}"))
            .resolve(today)
    };
    assert_eq!(day("today"), Some(today));
    assert_eq!(day("Yesterday"), Some(date("2026-02-28")));
    assert_eq!(day("2025-12-24"), Some(date("2025-12-24")));
    assert!("2025-13-01".parse::<LogDay>().is_err());
    assert!("tomorrow".parse::<LogDay>().is_err());
}

#[test]
fn seconds_format_as_hours_and_minutes() {
    assert_eq!(hours_minutes(0), "0:00");
    assert_eq!(hours_minutes(5400), "1:30");
    assert_eq!(hours_minutes(89), "0:01");
    assert_eq!(hours_minutes(36_000 + 59 * 60), "10:59");
    assert_eq!(hours_minutes(TimeDelta::hours(-1).num_seconds()), "0:00");
}
