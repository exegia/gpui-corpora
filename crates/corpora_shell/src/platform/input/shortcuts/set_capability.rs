#[allow(unused_imports)]
use super::*;

pub(crate) fn set_capability(state: &ShortcutState, cx: &mut App) {
    crate::capabilities::set(
        "global_shortcuts",
        crate::capabilities::CapabilityStatus {
            supported: cfg!(any(target_os = "macos", target_os = "windows")),
            enabled: state.registered,
            degraded: state.enabled && !state.registered,
            reason: state
                .last_error
                .as_ref()
                .map(|error| format!("shortcut unavailable: {error}").into())
                .or_else(|| {
                    if cfg!(any(target_os = "macos", target_os = "windows")) {
                        if !state.enabled {
                            Some("disabled by user setting".into())
                        } else {
                            None
                        }
                    } else {
                        Some("global shortcut service currently configured for macOS".into())
                    }
                }),
            last_error: state.last_error.clone().map(Into::into),
        },
        cx,
    );
}
