use reqwest::StatusCode;

use crate::api::error::{ApiError, ErrorBody};
use crate::errors::{ApiResultExt, KaneoError};

#[test]
fn json_error_bodies_keep_code_issues_and_permissions() {
    let text = r#"{"message":"Invalid body","code":"VALIDATION_ERROR",
        "issues":[{"path":"priority","message":"Invalid enum"}],
        "missingPermissions":["task:update"]}"#;
    let body = ErrorBody::parse(StatusCode::BAD_REQUEST, text);
    assert_eq!(body.code, "VALIDATION_ERROR");
    let shown = body.to_string();
    assert!(shown.contains("Invalid body (VALIDATION_ERROR)"));
    assert!(shown.contains("- priority: Invalid enum"));
    assert!(shown.contains("missing permissions: task:update"));
}

#[test]
fn non_json_bodies_fall_back_to_status_code() {
    let body = ErrorBody::parse(StatusCode::BAD_GATEWAY, "<html>bad gateway</html>");
    assert_eq!(body.code, "BAD_GATEWAY");
    assert_eq!(body.message, "<html>bad gateway</html>");
}

fn failure(status: StatusCode, code: &str) -> Result<(), ApiError> {
    Err(ApiError::Status {
        status,
        body: ErrorBody {
            message: "m".to_owned(),
            code: code.to_owned(),
            ..ErrorBody::default()
        },
    })
}

#[test]
fn hints_follow_the_stable_error_code() {
    let hint = |status, code| {
        failure(status, code)
            .with_action("Doing")
            .unwrap_err()
            .hint()
    };
    assert!(hint(StatusCode::NOT_FOUND, "NOT_FOUND").is_some_and(|h| h.contains("list")));
    assert!(hint(StatusCode::UNAUTHORIZED, "UNAUTHORIZED").is_some_and(|h| h.contains("login")));
    assert!(hint(StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED").is_some());
    assert_eq!(hint(StatusCode::CONFLICT, "CONFLICT"), None);
}

#[test]
fn actions_prefix_the_error_message() {
    let err = failure(StatusCode::NOT_FOUND, "NOT_FOUND")
        .with_action("Failed to get task")
        .unwrap_err();
    assert!(matches!(err, KaneoError::Api { .. }));
    assert_eq!(
        err.to_string(),
        "Failed to get task: 404 Not Found: m (NOT_FOUND)"
    );
}
