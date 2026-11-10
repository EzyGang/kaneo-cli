use crate::api::types::project::ProjectPosition;
use crate::cli::project_admin::reorder_body;
use crate::errors::KaneoError;

fn order(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| (*id).to_owned()).collect()
}

#[test]
fn positions_follow_the_given_order_ignoring_blanks() {
    let body = reorder_body(order(&[" b ", "", "a"])).unwrap_or_else(|e| panic!("body: {e}"));
    assert_eq!(
        body.projects,
        [
            ProjectPosition {
                id: "b".to_owned(),
                position: 0
            },
            ProjectPosition {
                id: "a".to_owned(),
                position: 1
            },
        ]
    );
}

#[test]
fn empty_order_is_rejected() {
    assert!(matches!(
        reorder_body(order(&["", " "])),
        Err(KaneoError::Usage(_))
    ));
}

#[test]
fn duplicate_ids_are_rejected() {
    assert!(matches!(
        reorder_body(order(&["a", "b", "a"])),
        Err(KaneoError::Usage(msg)) if msg.contains('a')
    ));
}
