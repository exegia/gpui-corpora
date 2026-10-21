use super::*;

pub struct NotificationsPage {
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl NotificationsPage {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            _subscriptions: vec![
                cx.observe_global_in::<inbox::NotificationInboxState>(window, |_, _, cx| {
                    cx.notify()
                }),
            ],
        }
    }
}

impl Render for NotificationsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = inbox::snapshot(cx);
        let empty = items.is_empty();
        let unread = items.iter().filter(|item| !item.read).count();
        let theme = cx.theme();
        div()
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                div()
                    .flex_none()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .rounded_t(gpui_kit::component::Theme::global(cx).radius)
                    .bg(theme.colors.sunken)
                    .child(
                        div()
                            .text_size(theme.text_size(TextSize::Md))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(crate::i18n::localize("notifications_title")),
                    )
                    .when(unread > 0, |header| {
                        header.child(Badge::new(unread.to_string()).tone(Tone::Accent))
                    })
                    .child(div().flex_1())
                    .when(!empty, |header| {
                        header
                            .child(
                                Button::new(
                                    "notifications-read-all",
                                    crate::i18n::localize("notifications_mark_all_read"),
                                )
                                .variant(ButtonVariant::Ghost)
                                .size(ControlSize::Sm)
                                .icon(IconName::CheckCheck)
                                .on_click(|_, _, cx| inbox::mark_all_read(cx)),
                            )
                            .child(
                                Button::new(
                                    "notifications-clear",
                                    crate::i18n::localize("notifications_clear_all"),
                                )
                                .variant(ButtonVariant::Ghost)
                                .size(ControlSize::Sm)
                                .icon(IconName::Trash2)
                                .on_click(|_, _, cx| inbox::clear_all(cx)),
                            )
                    }),
            )
            .child(if empty {
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        EmptyState::new(
                            "notifications-empty",
                            IconName::Bell,
                            crate::i18n::localize("notifications_title"),
                        )
                        .body(crate::i18n::localize("notifications_empty")),
                    )
                    .into_any_element()
            } else {
                ScrollArea::new("notifications-scroll")
                    .vertical()
                    .flex_1()
                    .min_h_0()
                    .children(items.into_iter().map(|item| notification(item, cx)))
                    .into_any_element()
            })
    }
}
