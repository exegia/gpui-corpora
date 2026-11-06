#[allow(unused_imports)]
use super::*;

/// Point-in-time accessibility state, stored as a global so observers (the
/// diagnostics page) re-render when it changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessibilitySnapshot {
    /// At least one open window is building an a11y tree right now, i.e.
    /// assistive technology is connected. Dynamic; see [`refresh`].
    pub bridge_enabled: bool,
    pub status: String,
    pub active_windows: usize,
    pub total_windows: usize,
}

impl Global for AccessibilitySnapshot {}

impl Default for AccessibilitySnapshot {
    fn default() -> Self {
        observe(0, 0)
    }
}
