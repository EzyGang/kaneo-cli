use serde::Deserialize;

/// The response of `GET /search`: one ranked, flat list of hits.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub results: Vec<SearchHit>,
    pub total_count: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub id: String,
    /// One of `task`, `project`, `workspace`, `comment`, or `activity`.
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub project_slug: Option<String>,
    #[serde(default)]
    pub task_number: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
}

/// Display order and section titles of search hit kinds.
const SECTIONS: [(&str, &str); 5] = [
    ("task", "Tasks"),
    ("project", "Projects"),
    ("workspace", "Workspaces"),
    ("comment", "Comments"),
    ("activity", "Activity"),
];

impl SearchResponse {
    /// Groups hits into titled sections, keeping the relevance order inside each section.
    ///
    /// Hits of unknown kinds are dropped.
    #[must_use]
    pub fn grouped(&self) -> Vec<(&'static str, Vec<&SearchHit>)> {
        SECTIONS
            .iter()
            .map(|(kind, title)| {
                let hits: Vec<&SearchHit> =
                    self.results.iter().filter(|h| h.kind == *kind).collect();
                (*title, hits)
            })
            .filter(|(_, hits)| !hits.is_empty())
            .collect()
    }
}
