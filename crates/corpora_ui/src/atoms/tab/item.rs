//! One tab in a [`Tabs`](super::Tabs) list: the native tab, the glass behind
//! it in the horizontal layout, and its close control.

use std::rc::Rc;

use gpui::{
    AnyElement, ElementId, FocusHandle, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, prelude::*, px,
};
use gpui_kit::{
    base::{
        Button as NativeButton, Tab as NativeTab, Transition,
        motion::{Presence, transition},
    },
    component::Theme as KitTheme,
};

use super::{
    appearance::{
        VERTICAL_TAB_LABEL_SIZE, VERTICAL_TAB_RADIUS, content_color, glass_spec,
    },
    semantics::{Tab, TabHandler, TabVariant},
    size::{VERTICAL_TAB_HEIGHT, VERTICAL_TAB_WIDTH},
};
use crate::{
    ely::{
        primitives::{Icon, IconName},
        theme::{ActiveTheme, ControlSize, IconSize, Radius, TextSize},
    },
    ui::material::GlassSurfaceExt as _,
};

/// Glass below this strength is not worth painting.
const MIN_GLASS_STRENGTH: f32 = 0.01;
/// Hover shows this fraction of the selected glass.
const HOVER_GLASS_STRENGTH: f32 = 0.6;

#[derive(IntoElement)]
pub(super) struct TabItem {
    pub(super) id: ElementId,
    pub(super) index: usize,
    pub(super) count: usize,
    pub(super) item: Tab,
    pub(super) active: bool,
    pub(super) variant: TabVariant,
    pub(super) focuses: Rc<Vec<FocusHandle>>,
    pub(super) on_change: Option<TabHandler>,
    pub(super) on_close: Option<TabHandler>,
}

impl RenderOnce for TabItem {
    fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let TabItem {
            id,
            index,
            count,
            item,
            active,
            variant,
            focuses,
            on_change,
            on_close,
        } = self;
        let (
            colors,
            horizontal_height,
            horizontal_padding,
            horizontal_min_width,
            horizontal_radius,
            horizontal_text_size,
            close_side,
        ) = {
            let theme = cx.theme();
            (
                theme.colors.clone(),
                theme.control_height(ControlSize::Lg) * 0.8,
                theme.control_padding(ControlSize::Md),
                theme.tab_width().0,
                theme.radius(Radius::Lg) * 1.1,
                theme.text_size(TextSize::Sm),
                theme.control_height(ControlSize::Sm) * 0.8,
            )
        };
        let (selection_duration, selection_easing) = {
            let motion = KitTheme::global(cx).motion_tokens();
            (motion.duration_fast, motion.easing_move.clone())
        };
        let vertical = variant == TabVariant::Vertical;
        let focus = focuses[index].clone();
        let key_focuses = focuses.clone();
        let click = on_change.clone();
        let key_change = on_change;
        let color = content_color(&colors, active);
        let closable = item.closable;
        let item_label = item.label.clone();

        let tab = NativeTab::new(index)
            .selected(active)
            .set_position(index + 1, count)
            .accessibility_label(item.label.clone())
            .track_focus(&focus)
            .flex_none()
            .text_color(color)
            .cursor_pointer()
            .focus_visible(move |style| style.border_color(colors.fg_muted))
            // The horizontal layout shows hover through its glass instead.
            .when(vertical, |tab| {
                tab.hover(move |style| {
                    if active {
                        style.bg(gpui::transparent_black())
                    } else {
                        style.bg(colors.fg_subtle.opacity(0.4))
                    }
                })
            })
            .on_click(move |_, window, cx| {
                focus.focus(window, cx);
                if let Some(change) = &click {
                    change(&index, window, cx);
                }
            })
            .on_key_down(move |event, window, cx| {
                let next = match event.keystroke.key.as_str() {
                    "left" => (index + count - 1) % count,
                    "right" => (index + 1) % count,
                    "home" => 0,
                    "end" => count - 1,
                    "enter" | "space" => index,
                    _ => return,
                };
                window.prevent_default();
                cx.stop_propagation();
                key_focuses[next].focus(window, cx);
                if let Some(change) = &key_change {
                    change(&next, window, cx);
                }
            });
        let tab = match variant {
            TabVariant::Vertical => tab
                .flex_col()
                .gap(px(4.))
                .w(px(VERTICAL_TAB_WIDTH))
                .h(px(VERTICAL_TAB_HEIGHT))
                .rounded(px(VERTICAL_TAB_RADIUS))
                .text_size(px(VERTICAL_TAB_LABEL_SIZE))
                .font_weight(gpui::FontWeight::BOLD),
            TabVariant::Horizontal => tab
                .flex()
                .items_center()
                .h(horizontal_height)
                .justify_start()
                .px(horizontal_padding)
                .when(closable, |tab| tab.pr_6())
                .min_w(horizontal_min_width)
                .gap_1p5()
                .rounded(horizontal_radius)
                .text_size(horizontal_text_size)
                .font_weight(gpui::FontWeight::MEDIUM),
        }
        .children(item.icon)
        .child(item.label);

        if vertical {
            return tab.into_any_element();
        }

        let timing = || Transition::new(selection_duration).easing(selection_easing.clone());
        let hover = window.use_keyed_state(
            (id.clone(), SharedString::from(format!("hover-{index}"))),
            cx,
            |_, _| false,
        );
        let hovered = *hover.read(cx);
        let hover_progress = transition(
            (id.clone(), format!("hover-glass-{index}")),
            if hovered { HOVER_GLASS_STRENGTH } else { 0. },
            timing(),
            window,
            cx,
        );
        let selection = Presence::new((id.clone(), format!("selection-{index}")), active)
            .transition(timing())
            .sample(window, cx);
        let strength = selection.progress.max(hover_progress);

        let tab: AnyElement = if strength > MIN_GLASS_STRENGTH {
            tab.glass_with(glass_spec(cx.theme(), strength))
                .into_any_element()
        } else {
            tab.into_any_element()
        };

        let group = SharedString::from(format!("tab-{index}"));
        let mut wrapper = div()
            .id((id.clone(), format!("item-{index}")))
            .relative()
            .group(group.clone())
            .flex_none()
            .on_hover(move |value, _, cx| {
                hover.update(cx, |state, cx| {
                    if *state != *value {
                        *state = *value;
                        cx.notify();
                    }
                })
            })
            .child(tab);
        if closable {
            let close = on_close;
            let close_top = (horizontal_height - close_side) * 0.5;
            let close_button = NativeButton::new((id.clone(), format!("close-{index}")))
                .accessibility_label(format!("Close {item_label}"))
                .disabled(close.is_none())
                .tab_stop(active && close.is_some())
                .w(close_side)
                .h(close_side)
                .rounded_full()
                .border_1()
                .border_color(gpui::transparent_black())
                .text_color(color)
                .cursor_pointer()
                .focus_visible(move |style| style.border_color(colors.accent))
                .hover(move |style| style.bg(colors.fg_subtle.opacity(0.3)))
                .active(move |style| style.bg(colors.active).text_color(colors.fg))
                .child(Icon::new(IconName::X).size(IconSize::Xs).color(color))
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    if let Some(close) = &close {
                        close(&index, window, cx);
                    }
                });
            wrapper = wrapper.child(
                div()
                    .absolute()
                    .right_1()
                    .top(close_top)
                    .when(!active, |slot| {
                        slot.invisible().group_hover(group, |style| style.visible())
                    })
                    .child(close_button),
            );
        }
        wrapper.into_any_element()
    }
}
