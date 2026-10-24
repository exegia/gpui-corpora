#[allow(unused_imports)]
use super::*;

/// Send a notification via the notification service without requiring a window handle.
pub(crate) fn dispatch_background_notification(title: &str, body: &str, cx: &mut gpui::App) {
    crate::notifications::inbox::record_attempt(
        crate::notifications::inbox::NotificationAttemptRecord {
            title: title.to_string(),
            body: body.to_string(),
            backend: crate::notifications::NotificationBackendKind::UiOnly,
            delivered_natively: false,
            degraded: false,
            error_summary: None,
            kind: crate::notifications::inbox::NotificationInboxKind::SettingsUpdate,
        },
        cx,
    );

    let state = match cx.try_global::<crate::notifications::NativeNotificationState>() {
        Some(s) => s.clone(),
        None => return,
    };
    let enabled_by_user = state.snapshot.enabled_by_user
        && state.snapshot.permission != crate::notifications::NotificationPermissionState::Denied;
    if !enabled_by_user {
        return;
    }

    let request = crate::notifications::NotificationRequest::background_worthy(title, body);
    let service = state.service.clone();

    cx.spawn(async move |cx| {
        let result = service.send(request, enabled_by_user).await;
        tracing::info!(
            target: "corpora::updater",
            backend = %result.backend_used,
            delivered = result.delivered_natively,
            "update notification send completed"
        );
        cx.update(|cx| {
            crate::notifications::apply_send_result_static(&result, cx);
        });
    })
    .detach();
}
