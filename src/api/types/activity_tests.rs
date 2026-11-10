use serde_json::json;

use crate::api::types::activity::event_summary;

#[test]
fn free_text_wins_and_is_flattened_to_one_line() {
    let data = json!({"oldStatus": "to-do"});
    assert_eq!(
        event_summary(Some("  first\nsecond "), Some(&data)),
        "first second"
    );
}

#[test]
fn blank_text_falls_back_to_scalar_event_fields() {
    let data = json!({
        "newStatus": "done",
        "nested": {"ignored": true},
        "empty": "",
        "count": 3,
        "flag": false,
        "missing": null
    });
    let summary = event_summary(Some("   "), Some(&data));
    let mut parts: Vec<&str> = summary.split(", ").collect();
    parts.sort_unstable();
    assert_eq!(parts, ["count: 3", "flag: false", "newStatus: done"]);
}

#[test]
fn missing_or_non_object_data_yields_empty_summary() {
    assert_eq!(event_summary(None, None), "");
    assert_eq!(event_summary(None, Some(&json!("text"))), "");
}
