#[allow(unused_imports)]
use super::*;

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn initialize(cx: &mut App) {
    let enabled = crate::app_state::with_config(cx, |c| c.global_shortcut_enabled);
    let state = ShortcutState {
        enabled,
        registered: false,
        accelerator: "Alt+Space".to_string(),
        last_error: None,
    };
    cx.set_global(state);
    // macOS hotkey events are forwarded by tray.rs, which is macOS-only.
    #[cfg(target_os = "windows")]
    spawn_hotkey_event_forwarder(cx);
    apply_enabled(enabled, cx);
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn initialize(cx: &mut App) {
    let state = ShortcutState {
        enabled: false,
        registered: false,
        accelerator: "Alt+Space".to_string(),
        last_error: Some("global shortcut service currently configured for macOS".to_string()),
    };
    set_capability(&state, cx);
    cx.set_global(state);
}
