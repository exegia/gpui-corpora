#[allow(unused_imports)]
use super::*;

// WM_QUIT posted as a thread message makes the worker's GetMessageW return 0,
// so it unregisters and drops the manager on its own thread.
#[cfg(target_os = "windows")]
pub(crate) fn stop_hotkey_worker() {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{PostThreadMessageW, WM_QUIT};

    let slot = HOTKEY_WORKER.get_or_init(|| Mutex::new(None));
    let Ok(mut guard) = slot.lock() else {
        return;
    };
    if let Some(worker) = guard.take() {
        if let Err(err) =
            unsafe { PostThreadMessageW(worker.thread_id, WM_QUIT, WPARAM(0), LPARAM(0)) }
        {
            tracing::warn!(target: LOG, error = %err, "failed to post WM_QUIT to hotkey worker");
        }
        let _ = worker.join.join();
    }
}
