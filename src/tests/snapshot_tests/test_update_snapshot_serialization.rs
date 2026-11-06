#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_update_snapshot_serialization() {
    let snapshot = UpdateSnapshot {
        status: UpdateStatus::Available {
            version: "2.1.0".to_string(),
            notes: "New features".to_string(),
        },
        current_version: "2.0.0".to_string(),
        last_check: Some("2026-06-04T12:00:00Z".to_string()),
        update_channel: "stable".to_string(),
        check_retry_count: 0,
        download_retry_count: 0,
        cached_manifest: None,
        cached_asset: None,
    };

    insta::assert_yaml_snapshot!(
            &snapshot,
    @r#"
        status:
          Available:
            version: 2.1.0
            notes: New features
        current_version: 2.0.0
        last_check: "2026-06-04T12:00:00Z"
        update_channel: stable
        check_retry_count: 0
        download_retry_count: 0
        "#
        );
}
