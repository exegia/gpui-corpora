#[allow(unused_imports)]
use super::*;

/// Fold a health-check outcome into a snapshot (boot task).
pub(crate) fn record_health_result(
    snapshot: &mut StorageSnapshot,
    result: Result<(), StorageError>,
) {
    match result {
        Ok(()) => snapshot.healthy = true,
        Err(err) => {
            snapshot.healthy = false;
            snapshot.last_error = Some(err.to_string());
        }
    }
}
