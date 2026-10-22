#[allow(unused_imports)]
use super::*;

/// Pure snapshot constructor so the status branches are testable without
/// open windows.
pub(crate) fn observe(active_windows: usize, total_windows: usize) -> AccessibilitySnapshot {
    let bridge_enabled = active_windows > 0;
    let status = if bridge_enabled {
        format!("accesskit bridge active in {active_windows} of {total_windows} windows")
    } else {
        "accesskit bridge ready; no assistive technology connected".to_string()
    };
    AccessibilitySnapshot {
        bridge_enabled,
        status,
        active_windows,
        total_windows,
    }
}
