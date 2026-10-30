#[allow(unused_imports)]
use super::*;

pub(crate) fn apply_send_result(result: &NotificationSendResult, cx: &mut App) {
    tracing::debug!(
        target: LOG,
        backend = %result.backend_used,
        degraded = result.degraded,
        delivered_natively = result.delivered_natively,
        error_summary = ?result.error_summary,
        "applying notification send result"
    );
    mutate_snapshot(cx, |snapshot| {
        snapshot.active_backend = result.backend_used;
        snapshot.last_backend_error = result.error_summary.clone();
        snapshot.degraded_reason = if result.degraded {
            result
                .error_summary
                .clone()
                .or_else(|| Some(crate::i18n::localize("notifications_degraded").into()))
        } else {
            None
        };
    });
}
