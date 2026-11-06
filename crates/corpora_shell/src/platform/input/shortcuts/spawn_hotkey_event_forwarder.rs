#[allow(unused_imports)]
use super::*;

// The drain thread parks on a blocking crossbeam recv and forwards over flume:
// `AsyncApp` is `!Send`, so it cannot `cx.update` (same shape as tray.rs).
#[cfg(target_os = "windows")]
pub(crate) fn spawn_hotkey_event_forwarder(cx: &mut App) {
    use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

    let (tx, rx) = flume::unbounded::<()>();
    let spawn_result = std::thread::Builder::new()
        .name("gpui-hotkey-events".into())
        .spawn(move || {
            let hotkey_rx = GlobalHotKeyEvent::receiver();
            loop {
                let Ok(event) = hotkey_rx.recv() else {
                    return;
                };
                // The crate emits Pressed and Released per press; forwarding
                // both would trigger the launcher twice per tap.
                if event.state() == HotKeyState::Pressed {
                    tracing::info!(target: LOG, source = "hotkey_alt_space", "Launcher trigger");
                    let _ = tx.send(());
                }
            }
        });
    if let Err(e) = spawn_result {
        tracing::error!(
            target: LOG,
            error = %e,
            "failed to spawn gpui-hotkey-events thread; global hotkey will not be delivered"
        );
        return;
    }

    cx.spawn(async move |cx| {
        loop {
            // Parked until a hotkey fires — no periodic wake-ups.
            if rx.recv_async().await.is_err() {
                break;
            }
            let _ = cx.update(crate::shell::open_palette);
        }
    })
    .detach();
}
