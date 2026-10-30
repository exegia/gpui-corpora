#[allow(unused_imports)]
use super::*;

/// Probe the FreeDesktop notification daemon at startup so Settings can show
/// "no daemon" immediately; the notify-rust probes park, so run off-thread.
pub fn refresh_daemon_state(cx: &mut App) {
    // Linux-only: macOS uses UNUserNotificationCenter and has no D-Bus daemon.
    #[cfg(target_os = "linux")]
    {
        let Some(runtime) = crate::services::tokio_runtime::handle(cx) else {
            tracing::debug!(target: LOG, "no tokio runtime available; skipping daemon probe");
            return;
        };
        tracing::debug!(target: LOG, "scheduling async notification-daemon probe");
        cx.spawn(async move |cx| {
            let probe = runtime
                .spawn_blocking(|| {
                    let info = notify_rust::get_server_information();
                    let caps = notify_rust::get_capabilities();
                    (info, caps)
                })
                .await;
            let (info, caps) = match probe {
                Ok((info, caps)) => (info.ok(), caps.ok()),
                Err(err) => {
                    tracing::warn!(target: LOG, error = %err, "daemon probe task failed");
                    (None, None)
                }
            };
            let daemon_present = info.is_some();
            let caps_display = caps.as_ref().map(|c| c.join(", ")).map(SharedString::from);
            tracing::info!(
                target: LOG,
                daemon_present,
                server = ?info.as_ref().map(|i| i.name.clone()),
                capabilities = ?caps,
                "notification-daemon probe completed"
            );
            cx.update(move |cx| {
                mutate_snapshot(cx, |snapshot| {
                    snapshot.daemon_capabilities = caps_display;
                    if daemon_present {
                        snapshot.degraded_reason = None;
                        snapshot.last_backend_error = None;
                    } else {
                        snapshot.degraded_reason = Some(
                            "No notification daemon owns org.freedesktop.Notifications — \
                             install notify-osd / dunst / mako"
                                .into(),
                        );
                        snapshot.last_backend_error = Some(
                            "org.freedesktop.Notifications is not reachable on the session bus"
                                .into(),
                        );
                    }
                });
            });
        })
        .detach();
    }

    #[cfg(not(target_os = "linux"))]
    {
        // No D-Bus notification daemon on macOS/Windows — nothing to probe.
        let _ = cx;
        tracing::debug!(target: LOG, "notification-daemon probe is Linux-only; skipped");
    }
}
