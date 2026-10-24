//! Controlled icon tabs with a shared moving glass selection surface.

use std::{rc::Rc, time::Duration};

use gpui::{
    AnyElement, App, BoxShadow, ElementId, FocusHandle, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StyleRefinement, Styled, Window, div, point, prelude::*, px,
};
use gpui_kit::base::{
    StyledExt, Tab as NativeTab, TestSupportExt,
    motion::{Spring, spring},
};

use crate::ely::theme::ActiveTheme;

const TAB_WIDTH: f32 = 63.;
const TAB_HEIGHT: f32 = 53.;
const TAB_GAP: f32 = 12.;

/// A tab's label and optional caller-supplied icon.
pub struct Tab {
    label: SharedString,
    icon: Option<AnyElement>,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
}

type ChangeHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

/// A centered tab list. Callers retain selection and render the associated panel.
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    label: SharedString,
    items: Vec<Tab>,
    selected: usize,
    autofocus: bool,
    on_change: Option<ChangeHandler>,
    style: StyleRefinement,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "Tabs".into(),
            items: Vec::new(),
            selected: 0,
            autofocus: false,
            on_change: None,
            style: StyleRefinement::default(),
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

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// Focuses the selected tab when the caller requests entry into this list.
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
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
        let count = self.items.len();
        let selected = self.selected.min(count.saturating_sub(1));
        let focuses: Vec<FocusHandle> = (0..count)
            .map(|index| {
                window
                    .use_keyed_state(
                        (
                            self.id.clone(),
                            SharedString::from(format!("focus-{index}")),
                        ),
                        cx,
                        |_, cx| cx.focus_handle(),
                    )
                    .read(cx)
                    .clone()
                    .tab_stop(index == selected)
            })
            .collect();
        if self.autofocus
            && let Some(focus) = focuses.get(selected)
        {
            focus.focus(window, cx);
        }
        let theme = cx.theme();
        let colors = theme.colors.clone();
        let dark = theme.is_dark();
        let tint = gpui::white().opacity(if dark { 16. / 255. } else { 18.8 / 255. });
        let x = spring(
            (self.id.clone(), "selection-position"),
            px(selected as f32 * (TAB_WIDTH + TAB_GAP)),
            Spring::new(Duration::from_millis(320)).with_epsilon(0.1),
            window,
            cx,
        );
        let mut list = div().relative().flex().gap(px(TAB_GAP)).h(px(TAB_HEIGHT));
        if count > 0 {
            // Bezel's Clear fallback: this GPUI renderer has no per-element backdrop lens.
            list = list.child(
                div()
                    .id((self.id.clone(), "selected-indicator"))
                    .test_support()
                    .absolute()
                    .left(x)
                    .top_0()
                    .w(px(TAB_WIDTH))
                    .h(px(TAB_HEIGHT))
                    .rounded(px(14.))
                    .bg(tint)
                    .border(px(1.4))
                    .border_color(gpui::white().opacity(0.26))
                    .shadow(vec![BoxShadow {
                        inset: false,
                        color: gpui::black().opacity(if dark { 0.16 } else { 0.04 }),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(3.),
                        spread_radius: px(0.),
                    }]),
            );
        }
        for (index, item) in self.items.into_iter().enumerate() {
            let focus = focuses[index].clone();
            let key_focuses = focuses.clone();
            let click = self.on_change.clone();
            let key_change = self.on_change.clone();
            let active = index == selected;
            list = list.child(
                NativeTab::new(index)
                    .selected(active)
                    .set_position(index + 1, count)
                    .accessibility_label(item.label.clone())
                    .track_focus(&focus)
                    .flex_none()
                    .flex_col()
                    .gap(px(4.))
                    .w(px(TAB_WIDTH))
                    .h(px(TAB_HEIGHT))
                    .rounded(px(14.))
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .text_size(px(9.))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(if active {
                        if dark { colors.fg } else { colors.on_accent }
                    } else {
                        colors.fg_muted
                    })
                    .cursor_pointer()
                    .focus_visible(move |style| style.border_color(colors.accent))
                    .hover(move |style| style.bg(colors.fg.opacity(0.04)))
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
                    })
                    .children(item.icon)
                    .child(item.label),
            );
        }
        div()
            .id(self.id)
            .test_support()
            .role(Role::TabList)
            .aria_label(self.label)
            .flex()
            .justify_center()
            .min_w_0()
            .child(list)
            .refine_style(&self.style)
    }
}

#[cfg(test)]
#[path = "tabs.test.rs"]
mod tests;
