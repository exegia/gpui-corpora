use corpora_ui::ely::buttons::IconButton;
use corpora_ui::ely::theme::{ControlSize, TextSize};
use gpui::FontWeight;
use gpui_kit::{Anchor, base::Selectable, component::popover::Popover, px};

use super::*;

/// Unread counts above this read as `99+`.
const MAX_UNREAD: usize = 99;

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
        let theme = cx.theme();
        let variant = if self.selected {
            ButtonVariant::Subtle
        } else {
            ButtonVariant::Ghost
        };
        let fill = theme.colors.danger;
        let foreground = corpora_ui::styles::palette::color(
            "on-danger",
            if theme.is_dark() {
                corpora_ui::ely::theme::Mode::Dark
            } else {
                corpora_ui::ely::theme::Mode::Light
            },
        );
        let height = theme.control_height(ControlSize::Sm) * 0.67;
        let count = if self.unread > MAX_UNREAD {
            format!("{MAX_UNREAD}+")
        } else {
            self.unread.to_string()
        };
        // One step below Ely's smallest size, which Ely's rolling digits cannot go under.
        let text_size = theme.text_size(TextSize::Xs) * (10.0 / 11.0);
        let pill = div()
            .flex()
            .items_center()
            .justify_center()
            .min_w(height)
            .h(height)
            .px_1()
            .rounded_full()
            .bg(fill)
            .text_color(foreground)
            .text_size(text_size)
            .font_weight(FontWeight::MEDIUM)
            .child(count);
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
                bell.child(div().absolute().top_neg_1().right_neg_1().child(pill))
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
