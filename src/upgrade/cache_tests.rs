use crate::upgrade::cache::is_newer;

#[test]
fn only_strictly_newer_releases_are_suggested() {
    assert!(is_newer("0.2.1", "0.2.0"));
    assert!(is_newer("v0.10.0", "0.9.9"));
    assert!(is_newer("1.0.0", "0.99.0"));
    assert!(!is_newer("0.1.8", "0.2.0"));
    assert!(!is_newer("0.2.0", "0.2.0"));
    assert!(!is_newer("garbage", "0.2.0"));
}
