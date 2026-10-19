#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub(crate) static HOTKEY_MANAGER: OnceLock<Mutex<Option<global_hotkey::GlobalHotKeyManager>>> =
    OnceLock::new();
