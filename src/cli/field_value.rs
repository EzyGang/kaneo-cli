use crate::api::types::custom_field::CustomField;
use crate::errors::KaneoError;

/// Finds a field by exact ID, or else by case-insensitive name.
pub fn resolve_field<'a>(
    fields: &'a [CustomField],
    arg: &str,
) -> Result<&'a CustomField, KaneoError> {
    if let Some(field) = fields.iter().find(|f| f.id == arg) {
        return Ok(field);
    }
    let mut matches = fields.iter().filter(|f| f.name.eq_ignore_ascii_case(arg));
    match (matches.next(), matches.next()) {
        (Some(field), None) => Ok(field),
        (Some(_), Some(_)) => Err(KaneoError::Usage(format!(
            "Several custom fields are named '{arg}'; pass the field ID instead"
        ))),
        (None, _) => Err(KaneoError::Usage(format!(
            "No custom field '{arg}' in this project"
        ))),
    }
}

/// Encodes user input as the string value the API stores for a field type.
/// Multiselect values are sent as a JSON array; comma-separated input is split.
#[must_use]
pub fn encode_value(field_type: &str, raw: &str) -> String {
    if field_type != "multiselect" || raw.trim_start().starts_with('[') {
        return raw.to_owned();
    }
    let items: Vec<&str> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    serde_json::Value::from(items).to_string()
}

/// Splits a comma-separated ID list, dropping blanks.
#[must_use]
pub fn split_ids(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
