use corpora_ui::atoms::badge::UnreadBadge;
use corpora_ui::ely::buttons::IconButton;
use gpui_kit::{Anchor, base::Selectable, component::popover::Popover, px};

use super::*;

/// The bell with its unread count. Kit's `Popover` marks it selected while open,
/// which Ely's `IconButton` has no state for, so this owns that flag. Ely's
/// `CountBadge` fixes its fill to the accent, so the count pill is drawn here.
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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let variant = if self.selected {
            ButtonVariant::Subtle
        } else {
            ButtonVariant::Ghost
        };
        div()
            .relative()
            .flex_none()
            .child(round(
                IconButton::new("open-notifications", IconName::Bell)
                    .variant(variant)
                    .tooltip(format!("Notifications ({} unread)", self.unread)),
                window,
                cx,
            ))
            .when(self.unread > 0, |bell| {
                bell.child(
                    div()
                        .absolute()
                        .top_neg_0p5()
                        .right_neg_0p5()
                        .child(UnreadBadge::new(self.unread)),
                )
            })
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
                .children(workspace.upgrade().and_then(|workspace| {
                    workspace
                        .read(cx)
                        .pages()
                        .map(|pages| pages.notifications.clone())
                }))
        })
}
