use crate::api::types::custom_field::CustomField;
use crate::cli::field_value::{encode_value, resolve_field, split_ids};

fn field(id: &str, name: &str) -> CustomField {
    CustomField {
        id: id.to_owned(),
        project_id: "p".to_owned(),
        name: name.to_owned(),
        field_type: "text".to_owned(),
        required: false,
        default_value: None,
        options: None,
        hidden_options: Vec::new(),
        position: 0.0,
    }
}

#[test]
fn resolves_by_id_then_case_insensitive_name() {
    let fields = vec![field("f1", "Estimate"), field("f2", "Team")];
    let by_id = resolve_field(&fields, "f2").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(by_id.id, "f2");
    let by_name = resolve_field(&fields, "estimate").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(by_name.id, "f1");
}

#[test]
fn rejects_unknown_and_ambiguous_names() {
    let fields = vec![field("f1", "Team"), field("f2", "team")];
    assert!(resolve_field(&fields, "TEAM").is_err());
    assert!(resolve_field(&fields, "missing").is_err());
}

#[test]
fn encodes_multiselect_as_json_array() {
    assert_eq!(encode_value("multiselect", "a, b ,,c"), r#"["a","b","c"]"#);
    assert_eq!(encode_value("multiselect", r#"["x"]"#), r#"["x"]"#);
    assert_eq!(encode_value("multiselect", ""), "[]");
}

#[test]
fn passes_other_types_through() {
    assert_eq!(encode_value("text", "a, b"), "a, b");
    assert_eq!(encode_value("number", "3"), "3");
}

#[test]
fn splits_id_lists() {
    assert_eq!(split_ids(" a,b ,, c"), vec!["a", "b", "c"]);
}
