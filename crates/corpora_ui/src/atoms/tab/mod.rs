//! Controlled tabs with horizontal and vertical glass layouts.

mod appearance;
mod semantics;
mod size;

pub use semantics::{Tab, TabVariant, Tabs};

use std::{rc::Rc, time::Duration};

use gpui::{
    App, ElementId, FocusHandle, IntoElement, ParentElement, RenderOnce, Role, SharedString, GlassEffect,
    StyleRefinement, Styled, Window, div, prelude::*, px,
};
use gpui_kit::{
    base::{
        Button as NativeButton, StyledExt, Tab as NativeTab, TestSupportExt, Transition,
        motion::{Presence, Spring, spring},
    },
    component::Theme as KitTheme,
};

use crate::{
    ely::{
        primitives::{Icon, IconName},
        theme::{ActiveTheme, ControlSize, IconSize, Radius, TextSize},
    },
    ui::glass,
};

use appearance::{VERTICAL_TAB_LABEL_SIZE, VERTICAL_TAB_RADIUS, content_color};

use size::{VERTICAL_TAB_GAP, VERTICAL_TAB_HEIGHT, VERTICAL_TAB_WIDTH};

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "Tabs".into(),
            items: Vec::new(),
            selected: Some(0),
            autofocus: false,
            on_change: None,
            on_close: None,
            style: StyleRefinement::default(),
            variant: TabVariant::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = Tab>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn selected_index(mut self, index: impl Into<Option<usize>>) -> Self {
        self.selected = index.into();
        self
    }

    /// Focuses the roving tab when the caller requests entry into this list.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Tabs {
            id,
            label,
            items,
            selected,
            autofocus,
            on_change,
            on_close,
            style,
            variant,
        } = self;
        let count = items.len();
        let selected = selected.map(|index| index.min(count.saturating_sub(1)));
        let focus_index = selected.unwrap_or_default();
        let focuses: Vec<FocusHandle> = (0..count)
            .map(|index| {
                window
                    .use_keyed_state(
                        (id.clone(), SharedString::from(format!("focus-{index}"))),
                        cx,
                        |_, cx| cx.focus_handle(),
                    )
                    .read(cx)
                    .clone()
                    .tab_stop(index == focus_index)
            })
            .collect();
        if autofocus && let Some(focus) = focuses.get(focus_index) {
            focus.focus(window, cx);
        }

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
                theme.control_height(ControlSize::Md),
                theme.control_padding(ControlSize::Md),
                theme.tab_width().0,
                theme.radius(Radius::Md),
                theme.text_size(TextSize::Sm),
                theme.control_height(ControlSize::Sm) * 0.8,
            )
        };
        let vertical = variant == TabVariant::Vertical;
        let close_top = (horizontal_height - close_side) * 0.5;
        let (selection_duration, selection_easing) = {
            let motion = KitTheme::global(cx).motion_tokens();
            (motion.duration_fast, motion.easing_move.clone())
        };
        let x = spring(
            (id.clone(), "selection-position"),
            px(focus_index as f32 * (VERTICAL_TAB_WIDTH + VERTICAL_TAB_GAP)),
            Spring::new(Duration::from_millis(320)).with_epsilon(0.1),
            window,
            cx,
        );
        let mut list = div().relative().flex().when(vertical, |list| {
            list.gap(px(VERTICAL_TAB_GAP)).h(px(VERTICAL_TAB_HEIGHT))
        });
        if vertical && count > 0 && selected.is_some() {
            list = list.child(
                div()
                    .id((id.clone(), "selected-indicator"))
                    .test_support()
                    .absolute()
                    .left(x)
                    .top_0()
                    .w(px(VERTICAL_TAB_WIDTH))
                    .h(px(VERTICAL_TAB_HEIGHT))
                    .rounded(px(VERTICAL_TAB_RADIUS))
                    .child(glass::clear_deep(px(VERTICAL_TAB_RADIUS))),
            );
        }

        for (index, item) in items.into_iter().enumerate() {
            let focus = focuses[index].clone();
            let key_focuses = focuses.clone();
            let click = on_change.clone();
            let key_change = on_change.clone();
            let active = selected == Some(index);
            let color = content_color(&colors, active);
            let closable = item.closable;
            let item_label = item.label.clone();
            let tab = NativeTab::new(index)
                .selected(active)
                .set_position(index + 1, count)
                .accessibility_label(item.label.clone())
                .track_focus(&focus)
                .flex_none()
                // .border_1()
                // .border_color(gpui::transparent_black())
                .text_color(color)
                .cursor_pointer()
                .focus_visible(move |style| style.border_color(colors.accent))
                .hover(move |style| {
                    if active {
                        style.bg(gpui::transparent_black())
                    } else {
                        style.bg(colors.fg_subtle.opacity(0.4))
                    }
                })
                // .active(move |style| {
                //     style
                //         .bg(colors.active)
                //         .text_color(colors.fg)
                //         .child(glass::clear(px(5.)))
                // })
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
                list = list.child(tab);
                continue;
            }

            let group = SharedString::from(format!("tab-{index}"));
            let selection = Presence::new((id.clone(), format!("selection-{index}")), active)
                .transition(Transition::new(selection_duration).easing(selection_easing.clone()))
                .sample(window, cx);
            let mut wrapper = div()
                .id((id.clone(), format!("item-{index}")))
                .relative()
                .group(group.clone())
                .flex_none();
            if selection.should_render() {
                wrapper = wrapper.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded(horizontal_radius)
                        .opacity(selection.progress)
                        .child(glass::clear_deep(px(8.))),
                );
            }
            wrapper = wrapper.child(tab);
            if closable {
                let close = on_close.clone();
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
            list = list.child(wrapper);
        }

        div()
            .id(id)
            .test_support()
            .role(Role::TabList)
            .aria_label(label)
            .flex()
            .gap_x_2()
            .when(vertical, |tabs| tabs.justify_center())
            .min_w_0()
            .child(list)
            .refine_style(&style)
    }
}

#[cfg(test)]
#[path = "tab.test.rs"]
mod tests;
