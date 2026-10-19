#[allow(unused_imports)]
use super::*;

pub(crate) fn portal_notification(
    request: &NotificationRequest,
) -> ashpd::desktop::notification::Notification {
    ashpd::desktop::notification::Notification::new(&request.title)
        .body(request.body.as_str())
        // Themed name resolved inside the sandbox through the flatpak icon
        // export; byte icons are deprecated by the portal spec.
        .icon(ashpd::desktop::Icon::with_names([APP_ICON_NAME]))
}
