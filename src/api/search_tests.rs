use serde_json::json;

use crate::api::types::search::SearchResponse;

#[test]
fn hits_are_grouped_by_kind_in_relevance_order() {
    let hit = |id: &str, kind: &str| json!({"id": id, "type": kind, "title": id});
    let response: SearchResponse = serde_json::from_value(json!({
        "results": [
            hit("c1", "comment"), hit("t1", "task"), hit("x", "unknown"),
            hit("t2", "task"), hit("p1", "project")
        ],
        "totalCount": 5,
        "searchQuery": "q"
    }))
    .unwrap_or_else(|e| panic!("fixture: {e}"));

    let groups: Vec<(&str, Vec<&str>)> = response
        .grouped()
        .into_iter()
        .map(|(title, hits)| (title, hits.iter().map(|h| h.id.as_str()).collect()))
        .collect();
    assert_eq!(
        groups,
        [
            ("Tasks", vec!["t1", "t2"]),
            ("Projects", vec!["p1"]),
            ("Comments", vec!["c1"])
        ]
    );
}
