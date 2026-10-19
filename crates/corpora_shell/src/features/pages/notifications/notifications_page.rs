#[allow(unused_imports)]
use super::*;

pub struct NotificationsPage {
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl NotificationsPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let subscriptions =
            vec![
                cx.observe_global_in::<inbox::NotificationInboxState>(window, |_, _, cx| {
                    cx.notify();
                }),
            ];
        Self {
            _subscriptions: subscriptions,
        }
    }
}

impl Render for NotificationsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = inbox::snapshot(cx);
        if items.is_empty() {
            return EmptyState::new(
                "notifications-empty",
                IconName::Bell,
                crate::i18n::localize("notifications_title"),
            )
            .body(crate::i18n::localize("notifications_empty"))
            .into_any_element();
        }
        NotificationCenter::new("notifications-center")
            .group(
                crate::i18n::localize("notifications_title"),
                items.into_iter().map(notification),
            )
            .on_read_all(|_, cx| inbox::mark_all_read(cx))
            .on_clear(|_, cx| inbox::clear_all(cx))
            .into_any_element()
    }
}
