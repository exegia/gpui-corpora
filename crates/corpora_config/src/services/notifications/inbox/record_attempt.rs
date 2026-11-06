#[allow(unused_imports)]
use super::*;

pub fn record_attempt(record_input: NotificationAttemptRecord, cx: &mut App) {
    record(
        NotificationInboxItem {
            id: NotificationId::new(),
            created_at: AppTimestamp::now(),
            title: record_input.title,
            body: record_input.body,
            read: false,
            backend: record_input.backend.to_string(),
            delivered_natively: record_input.delivered_natively,
            degraded: record_input.degraded,
            error_summary: record_input.error_summary,
            kind: record_input.kind,
        },
        cx,
    );
}
