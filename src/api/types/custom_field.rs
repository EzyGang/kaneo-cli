use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// The value type of a custom field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Number,
    Date,
    Dropdown,
    Boolean,
    Multiselect,
}

impl FieldType {
    /// Returns the value the API uses for this type.
    #[must_use]
    pub fn as_api_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
            Self::Dropdown => "dropdown",
            Self::Boolean => "boolean",
            Self::Multiselect => "multiselect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub id: String,
    pub project_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    /// Untyped in the spec; normally an array of option labels.
    #[serde(default)]
    pub options: Option<serde_json::Value>,
    #[serde(default)]
    pub hidden_options: Vec<String>,
    pub position: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldValue {
    pub id: String,
    pub task_id: String,
    pub field_id: String,
    pub value: Option<String>,
    pub field_name: String,
    pub field_position: f64,
    pub field_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomFieldBody {
    pub project_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<String>,
}

/// A partial update of a custom field; omitted fields stay unchanged.
/// `default_value: Some(None)` clears the default.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomFieldBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FieldType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_options: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFieldValueBody {
    pub task_id: String,
    pub field_id: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ReorderFieldsBody {
    pub fields: Vec<FieldPosition>,
}

#[derive(Debug, Serialize)]
pub struct FieldPosition {
    pub id: String,
    pub position: usize,
}

impl CustomField {
    /// Returns the option labels of a dropdown or multiselect field.
    #[must_use]
    pub fn option_labels(&self) -> Vec<String> {
        match &self.options {
            Some(serde_json::Value::Array(items)) => items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect(),
            _ => Vec::new(),
        }
    }
}
