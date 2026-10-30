#[allow(unused_imports)]
use super::*;

/// Regression guard: the boot health check lands after the synchronous
/// capability set, so every snapshot update must re-derive the status or a
/// healthy native boot freezes in as `degraded: true`. The full boot needs a
/// GPUI app context, so this replays the initialize sequence over the helpers.
#[test]
pub(crate) fn native_boot_capability_not_degraded_after_health_check_lands() {
    // What native `initialize` constructs after a successful `init_db`:
    // available, schema recorded, health check not yet run.
    let mut snapshot = StorageSnapshot {
        db_path: "/data/app.db".to_string(),
        available: true,
        schema_version: 3,
        last_migration_result: Some("schema version 3 ready".to_string()),
        ..StorageSnapshot::default()
    };

    let provisional = capability_status(true, &snapshot);
    assert!(provisional.degraded);

    record_health_result(&mut snapshot, Ok(()));
    let status = capability_status(true, &snapshot);
    assert!(status.supported);
    assert!(status.enabled);
    assert!(!status.degraded, "healthy native boot must not be degraded");
    assert_eq!(status.reason, None);
    assert_eq!(status.last_error, None);

    record_health_result(
        &mut snapshot,
        Err(StorageError::Sqlite(rusqlite::Error::InvalidQuery)),
    );
    let degraded = capability_status(true, &snapshot);
    assert!(degraded.degraded);
    assert!(degraded.reason.is_some());
    assert!(degraded.last_error.is_some());
}
