use corpora_ui::ely::buttons::IconButton;
use corpora_ui::ely::data_display::CountBadge;
use gpui_kit::{Anchor, base::Selectable, component::popover::Popover, px};

use super::*;

/// The bell with its unread count. Kit's `Popover` marks it selected while open,
/// which Ely's `IconButton` has no state for, so this owns that flag.
#[derive(IntoElement)]
struct NotificationsTrigger {
    unread: usize,
    selected: bool,
}

impl Selectable for NotificationsTrigger {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RenderOnce for NotificationsTrigger {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let variant = if self.selected {
            ButtonVariant::Subtle
        } else {
            ButtonVariant::Ghost
        };
        CountBadge::new("notifications-unread", self.unread).over(
            IconButton::new("open-notifications", IconName::Bell)
                .variant(variant)
                .tooltip(format!("Notifications ({} unread)", self.unread)),
        )
    }
}

pub(super) fn notifications(workspace: WeakEntity<Workspace>, cx: &App) -> Popover {
    let unread = crate::notifications::inbox::unread_count(cx);
    Popover::new("notifications-popover")
        .anchor(Anchor::TopRight)
        .arrow(false)
        .offset(px(-8.5))
        .p_0()
        .bg(cx.theme().colors.bg)
        .rounded(gpui_kit::component::Theme::global(cx).radius)
        .overflow_hidden()
        .trigger(NotificationsTrigger {
            unread,
            selected: false,
        })
        .content(move |_, window, cx| {
            let margin = cx.theme().window_margin() * 4.0;
            let size = window.viewport_size();
            let rem = cx.theme().base_rem();
            div()
                .w(rems(24.0).to_pixels(rem).min(size.width - margin))
                .h(rems(38.0).to_pixels(rem).min(size.height - margin))
                .children(
                    workspace
                        .upgrade()
                        .map(|workspace| workspace.read(cx).pages().notifications.clone()),
                )
        })
}
