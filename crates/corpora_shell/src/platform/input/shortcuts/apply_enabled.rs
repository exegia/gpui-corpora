#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub fn apply_enabled(enabled: bool, cx: &mut App) {
    use global_hotkey::{
        GlobalHotKeyManager,
        hotkey::{Code, HotKey, Modifiers},
    };

    let mut state = snapshot(cx);
    state.enabled = enabled;
    state.registered = false;
    state.last_error = None;

    let slot = HOTKEY_MANAGER.get_or_init(|| Mutex::new(None));
    if let Ok(mut manager_slot) = slot.lock() {
        *manager_slot = None;
        if enabled {
            match GlobalHotKeyManager::new() {
                Ok(manager) => {
                    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Space);
                    match manager.register(hotkey) {
                        Ok(()) => {
                            *manager_slot = Some(manager);
                            state.registered = true;
                        }
                        Err(err) => state.last_error = Some(err.to_string()),
                    }
                }
                Err(err) => state.last_error = Some(err.to_string()),
            }
        }
    } else {
        state.last_error = Some("failed to lock hotkey manager".to_string());
    }

    commit(state, cx);
}

#[cfg(target_os = "windows")]
pub fn apply_enabled(enabled: bool, cx: &mut App) {
    let mut state = snapshot(cx);
    state.enabled = enabled;
    state.registered = false;
    state.last_error = None;

    stop_hotkey_worker();
    if enabled {
        match spawn_hotkey_worker() {
            Ok(()) => state.registered = true,
            Err(err) => state.last_error = Some(err),
        }
    }

    commit(state, cx);
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn apply_enabled(enabled: bool, cx: &mut App) {
    let mut state = snapshot(cx);
    state.enabled = enabled;
    state.registered = false;
    state.last_error = if enabled {
        Some("global shortcut service currently configured for macOS".to_string())
    } else {
        None
    };
    commit(state, cx);
}
