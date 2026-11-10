use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub id: String,
    pub project_id: String,
    pub name: String,
    /// The value tasks use as their `status` while they are in this column.
    pub slug: String,
    pub position: f64,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_final: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
}

/// A partial update of a column, where omitted fields stay unchanged.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateColumnBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ReorderColumnsBody {
    pub columns: Vec<ColumnPosition>,
}

#[derive(Debug, Serialize)]
pub struct ColumnPosition {
    pub id: String,
    pub position: usize,
}
