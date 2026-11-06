#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub fn shutdown(cx: &mut App) {
    let slot = HOTKEY_MANAGER.get_or_init(|| Mutex::new(None));
    if let Ok(mut manager_slot) = slot.lock() {
        *manager_slot = None;
    }
    let mut state = snapshot(cx);
    state.registered = false;
    if state.enabled {
        state.last_error = Some("global shortcuts unregistered during shutdown".to_string());
    }
    commit(state, cx);
}

#[cfg(target_os = "windows")]
pub fn shutdown(cx: &mut App) {
    stop_hotkey_worker();
    let mut state = snapshot(cx);
    state.registered = false;
    if state.enabled {
        state.last_error = Some("global shortcuts unregistered during shutdown".to_string());
    }
    commit(state, cx);
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn shutdown(_cx: &mut App) {}
