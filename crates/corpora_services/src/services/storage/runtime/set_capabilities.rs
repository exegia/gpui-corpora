#[allow(unused_imports)]
use super::*;

/// Re-derive the capability-registry status from a snapshot. Backends settle
/// asynchronously after the synchronous boot, and the registry has no
/// observers — every snapshot update must call this again or a healthy boot
/// freezes in as `degraded: true`.
pub(crate) fn set_capabilities(supported: bool, snapshot: &StorageSnapshot, cx: &mut App) {
    crate::capabilities::set("storage", capability_status(supported, snapshot), cx);
}
