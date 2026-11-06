#[allow(unused_imports)]
use super::*;

pub fn set_native_notifications_enabled(enabled: bool, cx: &mut App) {
    tracing::info!(target: LOG, enabled, "native notifications user setting changed");
    crate::app_state::update_config(cx, |config| {
        config.native_notifications_enabled = enabled;
    });
    mutate_snapshot(cx, |snapshot| {
        snapshot.enabled_by_user = enabled;
        if !enabled {
            snapshot.degraded_reason =
                Some(crate::i18n::localize("notifications_degraded_disabled_by_user").into());
        } else {
            snapshot.degraded_reason = None;
        }
    });
    let snapshot = cx.global::<NativeNotificationState>().snapshot.clone();
    crate::capabilities::set(
        "native_notifications",
        crate::capabilities::CapabilityStatus {
            supported: true,
            enabled,
            degraded: snapshot.degraded_reason.is_some(),
            reason: snapshot.degraded_reason,
            last_error: snapshot.last_backend_error,
        },
        cx,
    );
    inbox::record_attempt(
        NotificationAttemptRecord {
            title: crate::i18n::localize("notifications_settings_updated"),
            body: crate::i18n::localize(if enabled {
                "notifications_native_enabled"
            } else {
                "notifications_native_disabled"
            }),
            backend: NotificationBackendKind::UiOnly,
            delivered_natively: false,
            degraded: !enabled,
            error_summary: None,
            kind: NotificationInboxKind::SettingsUpdate,
        },
        cx,
    );
}
