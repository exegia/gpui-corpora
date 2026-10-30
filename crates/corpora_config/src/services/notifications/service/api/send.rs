#[allow(unused_imports)]
use super::*;

pub fn send(request: NotificationRequest, cx: &mut App) {
    let state = cx.global::<NativeNotificationState>();
    let service = state.service.clone();
    let enabled_by_user = state.snapshot.enabled_by_user
        && state.snapshot.permission != NotificationPermissionState::Denied;
    let fallback_message = request.body.clone();
    let inbox_title = request.title.to_string();
    let inbox_body = request.body.to_string();
    // Capture before `request` is moved into service.send().
    let force_in_app_feedback = request.force_in_app_feedback;
    tracing::debug!(
        target: LOG,
        permission = ?state.snapshot.permission,
        enabled_by_user,
        "scheduling async notification send"
    );

    cx.spawn(async move |cx| {
        let result = service.send(request, enabled_by_user).await;
        tracing::info!(
            target: LOG,
            backend = %result.backend_used,
            degraded = result.degraded,
            delivered_natively = result.delivered_natively,
            error_summary = ?result.error_summary,
            "notification send completed"
        );
        // A native Ok is not proof a banner was shown (DND/mute/busy/fullscreen
        // suppress silently), so test affordances force in-app feedback.
        let should_show_in_app = force_in_app_feedback
            || (!result.delivered_natively
                && result.importance == NotificationImportance::ForegroundOnly);
        cx.update(move |cx| {
            apply_send_result(&result, cx);
            inbox::record_attempt(
                NotificationAttemptRecord {
                    title: inbox_title,
                    body: inbox_body,
                    backend: result.backend_used,
                    delivered_natively: result.delivered_natively,
                    degraded: result.degraded,
                    error_summary: result.error_summary.as_ref().map(ToString::to_string),
                    kind: NotificationInboxKind::Attempt,
                },
                cx,
            );
            if should_show_in_app {
                let feedback: SharedString = if force_in_app_feedback && result.delivered_natively {
                    // The daemon accepted the call but display is not guaranteed —
                    // name the common silent-suppression causes, don't feign success.
                    format!(
                        "Sent to the {} notification daemon. If no banner appeared, \
                         check Do Not Disturb and the per-app setting for \"{}\".",
                        result.backend_used,
                        crate::notifications::DESKTOP_ENTRY_ID,
                    )
                    .into()
                } else {
                    // Native delivery failed for a known reason — surface that
                    // instead of silently re-showing the body.
                    result
                        .error_summary
                        .as_ref()
                        .map(|err| {
                            format!(
                                "{}: {err}",
                                crate::i18n::localize("notifications_native_unavailable")
                            )
                            .into()
                        })
                        .unwrap_or(fallback_message)
                };
                push_in_app_feedback(feedback, cx);
            }
        });
    })
    .detach();
}
