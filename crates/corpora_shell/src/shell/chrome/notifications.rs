use ely_gpui_component::{data_display::CountBadge, overlays::Popover};

use super::*;

pub(super) fn notifications(workspace: WeakEntity<Workspace>, cx: &App) -> Popover {
    let unread = crate::notifications::inbox::unread_count(cx);
    Popover::with_opener(
        "notifications-popover",
        move |toggle| {
            div()
                .id("notifications-trigger")
                .on_click(move |_, window, cx| toggle(window, cx))
                .child(
                    CountBadge::new("notifications-unread", unread).over(
                        IconButton::new("open-notifications", IconName::Bell)
                            .tooltip(format!("Notifications ({unread} unread)")),
                    ),
                )
        },
        move |_, window, cx| {
            let margin = cx.theme().window_margin() * 4.0;
            let size = window.viewport_size();
            let rem = cx.theme().base_rem();
            div()
                .w(rems(38.0).to_pixels(rem).min(size.width - margin))
                .h(rems(28.0).to_pixels(rem).min(size.height - margin))
                .children(
                    workspace
                        .upgrade()
                        .map(|workspace| workspace.read(cx).pages.notifications.clone()),
                )
        },
    )
}
