use gpui::{App, BorrowAppContext as _, Global};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::sync::{Mutex, OnceLock};

mod apply_enabled;
mod commit;
mod hotkey_manager;
mod hotkey_worker;
mod initialize;
mod log;
mod set_capability;
mod shortcut_state;
mod shutdown;
mod snapshot;
mod spawn_hotkey_event_forwarder;
mod spawn_hotkey_worker;
mod stop_hotkey_worker;

#[cfg(target_os = "macos")]
pub use apply_enabled::apply_enabled;
#[cfg(target_os = "windows")]
pub use apply_enabled::apply_enabled;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use apply_enabled::apply_enabled;
use commit::commit;
#[cfg(target_os = "macos")]
use hotkey_manager::HOTKEY_MANAGER;
#[cfg(target_os = "windows")]
use hotkey_worker::HOTKEY_WORKER;
#[cfg(target_os = "windows")]
use hotkey_worker::HotkeyWorker;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use initialize::initialize;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use initialize::initialize;
#[cfg(target_os = "windows")]
use log::LOG;
use set_capability::set_capability;
pub use shortcut_state::ShortcutState;
#[cfg(target_os = "macos")]
pub use shutdown::shutdown;
#[cfg(target_os = "windows")]
pub use shutdown::shutdown;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use shutdown::shutdown;
pub use snapshot::snapshot;
#[cfg(target_os = "windows")]
use spawn_hotkey_event_forwarder::spawn_hotkey_event_forwarder;
#[cfg(target_os = "windows")]
use spawn_hotkey_worker::spawn_hotkey_worker;
#[cfg(target_os = "windows")]
use stop_hotkey_worker::stop_hotkey_worker;
