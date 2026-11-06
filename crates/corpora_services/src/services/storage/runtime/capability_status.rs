#[allow(unused_imports)]
use super::*;

/// Pure snapshot → capability-status derivation, unit-tested in storage.test.rs
/// (the full boot ordering needs a GPUI app context tests can't construct).
pub(crate) fn capability_status(
    supported: bool,
    snapshot: &StorageSnapshot,
) -> crate::capabilities::CapabilityStatus {
    crate::capabilities::CapabilityStatus {
        supported,
        enabled: snapshot.available,
        degraded: snapshot.last_error.is_some() || !snapshot.healthy,
        reason: snapshot
            .last_error
            .as_ref()
            .map(|err| format!("storage issue: {err}").into()),
        last_error: snapshot.last_error.clone().map(Into::into),
    }
}
