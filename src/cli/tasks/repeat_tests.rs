use crate::api::types::recurrence::{Frequency, Recurrence};
use crate::cli::tasks::repeat::{RecurrenceArgs, RepeatArgs};
use crate::errors::KaneoError;

fn rule(every: Option<u32>, on: &[&str]) -> RecurrenceArgs {
    RecurrenceArgs {
        every,
        on: on.iter().map(|day| (*day).to_owned()).collect(),
        time_zone: Some("Europe/Berlin".to_owned()),
    }
}

fn built(args: &RecurrenceArgs, frequency: Frequency) -> Recurrence {
    args.build(Some(frequency))
        .unwrap_or_else(|e| panic!("build failed: {e}"))
        .unwrap_or_else(|| panic!("expected a rule"))
}

fn assert_usage(result: Result<Option<Recurrence>, KaneoError>) {
    match result {
        Err(KaneoError::Usage(_)) => {}
        other => panic!("expected a usage error, got {other:?}"),
    }
}

#[test]
fn weekday_names_map_to_sorted_unique_numbers() {
    let args = rule(Some(2), &["Thu", "monday", "sun", "thurs"]);
    let recurrence = built(&args, Frequency::Weekly);
    assert_eq!(recurrence.weekdays, Some(vec![0, 1, 4]));
    assert_eq!(recurrence.interval, 2);
    assert_eq!(recurrence.time_zone, "Europe/Berlin");
}

#[test]
fn unknown_or_ambiguous_weekdays_are_rejected() {
    assert_usage(rule(None, &["funday"]).build(Some(Frequency::Weekly)));
    assert_usage(rule(None, &["t"]).build(Some(Frequency::Weekly)));
}

#[test]
fn interval_defaults_to_one_and_must_be_within_bounds() {
    assert_eq!(built(&rule(None, &[]), Frequency::Daily).interval, 1);
    assert_eq!(built(&rule(Some(99), &[]), Frequency::Daily).interval, 99);
    assert_usage(rule(Some(0), &[]).build(Some(Frequency::Daily)));
    assert_usage(rule(Some(100), &[]).build(Some(Frequency::Daily)));
}

#[test]
fn weekdays_require_a_weekly_rule() {
    assert_usage(rule(None, &["mon"]).build(Some(Frequency::Monthly)));
}

#[test]
fn rule_flags_without_frequency_are_rejected() {
    assert_usage(rule(Some(2), &[]).build(None));
    let none = RecurrenceArgs::default()
        .build(None)
        .unwrap_or_else(|e| panic!("build failed: {e}"));
    assert_eq!(none, None);
}

#[test]
fn missing_time_zone_falls_back_to_a_detected_zone() {
    let args = RecurrenceArgs::default();
    assert!(!built(&args, Frequency::Yearly).time_zone.is_empty());
}

#[test]
fn stop_clears_the_rule_and_frequency_is_otherwise_required() {
    let mut args = RepeatArgs {
        task: "KAN-1".to_owned(),
        frequency: None,
        rule: RecurrenceArgs::default(),
        stop: true,
    };
    let cleared = args
        .recurrence()
        .unwrap_or_else(|e| panic!("rule failed: {e}"));
    assert_eq!(cleared, None);

    args.stop = false;
    assert_usage(args.recurrence());

    args.frequency = Some(Frequency::Weekly);
    let set = args
        .recurrence()
        .unwrap_or_else(|e| panic!("rule failed: {e}"));
    assert_eq!(set.map(|r| r.frequency), Some(Frequency::Weekly));
}
