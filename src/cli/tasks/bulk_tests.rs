use crate::api::types::priority::Priority;
use crate::api::types::task_ops::BulkOperation;
use crate::cli::tasks::bulk::{BulkAction, BulkArgs};
use crate::errors::KaneoError;

fn action(args: BulkArgs) -> BulkAction {
    args.action()
        .unwrap_or_else(|e| panic!("action failed: {e}"))
}

fn expect(operation: BulkOperation, value: Option<&str>) -> BulkAction {
    BulkAction {
        operation,
        value: value.map(str::to_owned),
    }
}

#[test]
fn value_flags_map_to_their_operation() {
    let status = BulkArgs {
        status: Some("done".to_owned()),
        ..BulkArgs::default()
    };
    assert_eq!(
        action(status),
        expect(BulkOperation::UpdateStatus, Some("done"))
    );

    let priority = BulkArgs {
        priority: Some(Priority::NoPriority),
        ..BulkArgs::default()
    };
    let expected = expect(BulkOperation::UpdatePriority, Some("no-priority"));
    assert_eq!(action(priority), expected);

    let add = BulkArgs {
        add_label: Some("l1".to_owned()),
        ..BulkArgs::default()
    };
    assert_eq!(action(add), expect(BulkOperation::AddLabel, Some("l1")));

    let remove = BulkArgs {
        remove_label: Some("l2".to_owned()),
        ..BulkArgs::default()
    };
    assert_eq!(
        action(remove),
        expect(BulkOperation::RemoveLabel, Some("l2"))
    );
}

#[test]
fn assignee_flags_set_or_clear_the_assignee() {
    let assign = BulkArgs {
        assignee: Some("me".to_owned()),
        ..BulkArgs::default()
    };
    assert_eq!(
        action(assign),
        expect(BulkOperation::UpdateAssignee, Some("me"))
    );

    let unassign = BulkArgs {
        unassign: true,
        ..BulkArgs::default()
    };
    assert_eq!(
        action(unassign),
        expect(BulkOperation::UpdateAssignee, None)
    );
}

#[test]
fn due_none_clears_the_due_date() {
    let set = BulkArgs {
        due: Some("2026-12-01".to_owned()),
        ..BulkArgs::default()
    };
    let expected = expect(BulkOperation::UpdateDueDate, Some("2026-12-01"));
    assert_eq!(action(set), expected);

    let clear = BulkArgs {
        due: Some("none".to_owned()),
        ..BulkArgs::default()
    };
    assert_eq!(action(clear), expect(BulkOperation::UpdateDueDate, None));
}

#[test]
fn delete_sends_no_value() {
    let delete = BulkArgs {
        delete: true,
        ..BulkArgs::default()
    };
    assert_eq!(action(delete), expect(BulkOperation::Delete, None));
}

#[test]
fn missing_operation_is_a_usage_error() {
    match BulkArgs::default().action() {
        Err(KaneoError::Usage(_)) => {}
        other => panic!("expected a usage error, got {other:?}"),
    }
}
