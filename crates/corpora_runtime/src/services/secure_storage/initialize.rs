#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let available = keyring::Entry::new("corpora", "availability-check").is_ok();
    let snapshot = SecureStorageSnapshot {
        available,
        last_error: if available {
            None
        } else {
            Some("keyring entry initialization unavailable".to_string())
        },
    };
    cx.set_global(snapshot.clone());
    let status = if snapshot.available {
        crate::capabilities::CapabilityStatus::supported_enabled()
    } else {
        crate::capabilities::CapabilityStatus::error(
            snapshot
                .last_error
                .as_deref()
                .unwrap_or("secure storage unavailable"),
        )
    };
    crate::capabilities::set("secure_storage", status, cx);
}
