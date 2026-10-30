//! Controlled tabs with horizontal and vertical glass layouts.

mod appearance;
mod item;
mod semantics;
mod size;

pub use semantics::{Tab, TabVariant, Tabs};

use std::{rc::Rc, time::Duration};

use gpui::{
    App, ElementId, FocusHandle, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StyleRefinement, Styled, Window, div, prelude::*, px,
};
use gpui_kit::base::{
    StyledExt, TestSupportExt,
    motion::{Spring, spring},
};

use crate::ui::glass;

use appearance::{HORIZONTAL_TAB_GAP, VERTICAL_TAB_RADIUS};
use item::TabItem;
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

        let vertical = variant == TabVariant::Vertical;
        let x = spring(
            (id.clone(), "selection-position"),
            px(focus_index as f32 * (VERTICAL_TAB_WIDTH + VERTICAL_TAB_GAP)),
            Spring::new(Duration::from_millis(320)).with_epsilon(0.1),
            window,
            cx,
        );
        let mut list = div()
            .relative()
            .flex()
            .when(vertical, |list| {
                list.gap(px(VERTICAL_TAB_GAP)).h(px(VERTICAL_TAB_HEIGHT))
            })
            .when(!vertical, |list| list.gap(px(HORIZONTAL_TAB_GAP)));

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

        let focuses = Rc::new(focuses);
        for (index, item) in items.into_iter().enumerate() {
            list = list.child(TabItem {
                id: id.clone(),
                index,
                count,
                item,
                active: selected == Some(index),
                variant,
                focuses: focuses.clone(),
                on_change: on_change.clone(),
                on_close: on_close.clone(),
            });
        }

        div()
            .id(id)
            .test_support()
            .role(Role::TabList)
            .aria_label(label)
            .flex()
            .gap_2()
            .when(vertical, |tabs| tabs.justify_center())
            .min_w_0()
            .child(list)
            .refine_style(&style)
    }
}

#[cfg(test)]
#[path = "tab.test.rs"]
mod tests;
