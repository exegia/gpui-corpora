#[allow(unused_imports)]
use super::*;

// On Windows the manager owns a hidden window created on the calling thread:
// WM_HOTKEY dispatches only while that thread pumps messages and Drop
// (DestroyWindow) must run on that same thread, so the manager's whole
// lifecycle is confined to one worker thread.
#[cfg(target_os = "windows")]
pub(crate) static HOTKEY_WORKER: OnceLock<Mutex<Option<HotkeyWorker>>> = OnceLock::new();

#[cfg(target_os = "windows")]
pub(crate) struct HotkeyWorker {
    pub(crate) thread_id: u32,
    pub(crate) join: std::thread::JoinHandle<()>,
}
