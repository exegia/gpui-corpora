#[allow(unused_imports)]
use super::*;

pub fn request_permission(cx: &mut App) {
    let service = cx.global::<NativeNotificationState>().service.clone();
    tracing::debug!(target: LOG, "scheduling async permission request");
    cx.spawn(async move |cx| {
        let permission = service.request_permission().await;
        tracing::info!(target: LOG, ?permission, "permission request completed");
        let message = format!(
            "{}: {}",
            crate::i18n::localize("notifications_permission_status"),
            permission.label()
        );
        cx.update(move |cx| {
            mutate_snapshot(cx, |snapshot| {
                snapshot.permission = permission;
            });
            inbox::record_attempt(
                NotificationAttemptRecord {
                    title: crate::i18n::localize("notifications_permission_updated"),
                    body: message.to_string(),
                    backend: NotificationBackendKind::UiOnly,
                    delivered_natively: false,
                    degraded: false,
                    error_summary: None,
                    kind: NotificationInboxKind::PermissionUpdate,
                },
                cx,
            );
            push_in_app_feedback(message, cx);
        });
    })
    .detach();
}
