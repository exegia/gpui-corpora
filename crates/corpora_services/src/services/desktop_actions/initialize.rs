#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let mut snapshot = DesktopActionsSnapshot {
        clipboard_available: Clipboard::new().is_ok(),
        ..DesktopActionsSnapshot::default()
    };
    if !snapshot.clipboard_available {
        snapshot.last_error = Some("clipboard backend unavailable".to_string());
    }
    let status = match snapshot.last_error.as_deref() {
        Some(err) => crate::capabilities::CapabilityStatus::degraded(err),
        None => crate::capabilities::CapabilityStatus::supported_enabled(),
    };
    crate::capabilities::set("desktop_actions", status, cx);
    cx.set_global(DesktopActionsState {
        snapshot,
        inner: Arc::new(Mutex::new(DesktopActionsInner {
            next_watcher_id: 1,
            watchers: BTreeMap::new(),
        })),
    });
}
