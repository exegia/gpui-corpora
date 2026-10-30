#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "windows")]
pub(crate) fn spawn_hotkey_worker() -> Result<(), String> {
    use global_hotkey::{
        GlobalHotKeyManager,
        hotkey::{Code, HotKey, Modifiers},
    };
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, MSG, TranslateMessage,
    };

    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<u32, String>>();
    let join = std::thread::Builder::new()
        .name("gpui-global-hotkey".into())
        .spawn(move || {
            let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Space);
            let manager = GlobalHotKeyManager::new()
                .and_then(|manager| manager.register(hotkey).map(|_| manager));
            match manager {
                Ok(manager) => {
                    let thread_id = unsafe { GetCurrentThreadId() };
                    let _ = ready_tx.send(Ok(thread_id));
                    let mut msg = MSG::default();
                    // GetMessageW returns 0 on WM_QUIT and -1 on error; both end the pump.
                    while unsafe { GetMessageW(&mut msg, None, 0, 0) }.0 > 0 {
                        unsafe {
                            let _ = TranslateMessage(&msg);
                            DispatchMessageW(&msg);
                        }
                    }
                    let _ = manager.unregister(hotkey);
                }
                Err(err) => {
                    let _ = ready_tx.send(Err(err.to_string()));
                }
            }
        })
        .map_err(|err| format!("failed to spawn hotkey worker thread: {err}"))?;

    let worker = match ready_rx.recv() {
        Ok(Ok(thread_id)) => HotkeyWorker { thread_id, join },
        Ok(Err(err)) => {
            let _ = join.join();
            return Err(err);
        }
        Err(_) => {
            let _ = join.join();
            return Err("hotkey worker exited before reporting readiness".to_string());
        }
    };

    let slot = HOTKEY_WORKER.get_or_init(|| Mutex::new(None));
    match slot.lock() {
        Ok(mut guard) => {
            *guard = Some(worker);
            Ok(())
        }
        Err(_) => Err("failed to lock hotkey worker slot".to_string()),
    }
}
