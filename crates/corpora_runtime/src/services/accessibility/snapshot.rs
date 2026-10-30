#[allow(unused_imports)]
use super::*;

/// The last stored snapshot. Call [`refresh`] first when current counts
/// matter.
pub fn snapshot(cx: &App) -> AccessibilitySnapshot {
    cx.try_global::<AccessibilitySnapshot>()
        .cloned()
        .unwrap_or_default()
}
