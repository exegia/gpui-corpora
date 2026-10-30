use super::*;

pub(crate) fn notification(item: NotificationInboxItem, cx: &App) -> impl IntoElement + use<> {
    let theme = cx.theme();
    let colors = &theme.colors;
    let id = item.id;
    let unread = !item.read;
    let icon = if item.error_summary.is_some() || item.degraded {
        IconName::TriangleAlert
    } else {
        IconName::Bell
    };
    div()
        .id(SharedString::from(format!("inbox-item-{id}")))
        .relative()
        .flex()
        .items_start()
        .gap_3()
        .px_4()
        .py_3()
        .cursor_pointer()
        .hover(|row| row.bg(colors.hover))
        .on_click(move |_, _, cx| inbox::mark_read(id, cx))
        .when(unread, |row| {
            row.child(
                div()
                    .absolute()
                    .left_1p5()
                    .top_0()
                    .mt_3()
                    .h(theme.control_height(ControlSize::Lg))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .size(theme.status_dot())
                            .rounded_full()
                            .bg(colors.accent),
                    ),
            )
        })
        .child(
            div()
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .size(theme.control_height(ControlSize::Lg))
                .rounded_full()
                .bg(colors.sunken)
                .child(Icon::new(icon).size(IconSize::Md).color(colors.fg_muted)),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_size(theme.text_size(TextSize::Sm))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(colors.fg)
                                .child(item.title),
                        )
                        .child(
                            div()
                                .flex_none()
                                .text_size(theme.text_size(TextSize::Xs))
                                .text_color(colors.fg_subtle)
                                .child(RelativeTime::new(
                                    SharedString::from(format!("inbox-time-{id}")),
                                    item.created_at
                                        .to_rfc3339()
                                        .parse()
                                        .expect("valid notification timestamp"),
                                )),
                        ),
                )
                .child(
                    div()
                        .text_size(theme.text_size(TextSize::Xs))
                        .text_color(colors.fg_muted)
                        .child(item.body),
                )
                .when_some(item.error_summary, |body, error| {
                    body.child(
                        div()
                            .text_size(theme.text_size(TextSize::Xs))
                            .text_color(colors.fg_muted)
                            .child(error),
                    )
                })
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_1()
                        .child(Badge::new(item.backend))
                        .child(
                            Badge::new(crate::i18n::localize(if unread {
                                "notifications_status_unread"
                            } else {
                                "notifications_status_read"
                            }))
                            .tone(if unread {
                                Tone::Accent
                            } else {
                                Tone::default()
                            }),
                        ),
                ),
        )
}
