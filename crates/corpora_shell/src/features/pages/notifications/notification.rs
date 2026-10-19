#[allow(unused_imports)]
use super::*;

pub(crate) fn notification(item: NotificationInboxItem) -> Notification {
    let mut body = format!("{}\n{}", item.body, item.summary_line());
    if let Some(error) = &item.error_summary {
        body.push_str(&format!(
            "\n{} {error}",
            crate::i18n::localize("notifications_error")
        ));
    }
    let icon = if item.error_summary.is_some() || item.degraded {
        IconName::TriangleAlert
    } else {
        IconName::Bell
    };
    Notification::new(
        SharedString::from(format!("inbox-item-{}", item.id)),
        icon,
        item.title,
    )
    .body(body)
    .time(item.created_at.to_rfc3339())
    .unread(!item.read)
}
