use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, Window, div,
    prelude::*,
};
use gpui_kit::base::{
    Button, Selectable,
    motion::{Transition, transition},
};

use crate::ely::{
    primitives::Tooltip,
    theme::{ActiveTheme, Radius},
};

/// A small ghost button that a supplied menu wraps as its trigger.
#[derive(IntoElement)]
pub struct BreadcrumbTrigger {
    id: ElementId,
    label: SharedString,
    content: Option<AnyElement>,
    tooltip: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    open: bool,
    popup: bool,
    disabled: bool,
    icon_only: bool,
    menu: bool,
}

impl BreadcrumbTrigger {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            content: None,
            tooltip: None,
            on_click: None,
            open: false,
            popup: false,
            disabled: false,
            icon_only: false,
            menu: false,
        }
    }

    pub fn tooltip(mut self, text: impl Into<SharedString>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub(super) fn label(&self) -> SharedString {
        self.label.clone()
    }

    pub(super) fn popup(
        mut self,
        open: bool,
        activate: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        let on_click = self.on_click.take();
        self.open = open;
        self.popup = true;
        self.on_click = Some(Box::new(move |event, window, cx| {
            if let Some(on_click) = &on_click {
                on_click(event, window, cx);
            }
            activate(event, window, cx);
        }));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub(super) fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
    pub(super) fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }
    pub(super) fn menu(mut self) -> Self {
        self.menu = true;
        self
    }
}

impl Selectable for BreadcrumbTrigger {
    fn selected(mut self, selected: bool) -> Self {
        self.open = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.open
    }
}

impl RenderOnce for BreadcrumbTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let hovered = window.use_keyed_state((self.id.clone(), "hover"), cx, |_, _| false);
        let focus = window
            .use_keyed_state((self.id.clone(), "focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let background = transition(
            (self.id.clone(), "background"),
            if !self.disabled && (self.open || *hovered.read(cx)) {
                1.0_f32
            } else {
                0.0
            },
            Transition::new(std::time::Duration::from_millis(150)).ease(
                gpui_kit::base::animation::cubic_bezier(0.22, 1.0, 0.36, 1.0),
            ),
            window,
            cx,
        );
        let theme = cx.theme();
        let colors = &theme.colors;
        let into = focus.clone();
        let button = Button::new(self.id.clone())
            .accessibility_label(self.label.clone())
            .when(self.popup, |button| button.aria_expanded(self.open))
            .disabled(self.disabled)
            .track_focus(&focus)
            .tab_stop(!self.disabled)
            .h_7()
            .px(gpui::rems(0.5625))
            .gap_1p5()
            .rounded(theme.radius(Radius::Lg) * 0.8)
            .border_1()
            .border_color(gpui::transparent_black())
            .text_sm()
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(colors.fg)
            .bg(colors.hover.opacity(background))
            .when(!self.disabled, |button| {
                button
                    .active(|style| style.bg(colors.active))
                    .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                        into.focus(window, cx)
                    })
            })
            .when(self.menu, |button| button.pr_1())
            .when(self.icon_only, |button| button.size_6().p_0())
            .when(self.disabled, |button| button.opacity(0.64))
            .when_some(self.tooltip, |button, text| {
                button.tooltip(Tooltip::text(text))
            })
            .when_some(self.on_click, |button, handler| button.on_click(handler))
            .child(
                self.content
                    .unwrap_or_else(|| self.label.into_any_element()),
            );
        div()
            .id((self.id, "hover-region"))
            .flex()
            .child(button)
            .on_hover(move |value, _, cx| {
                hovered.update(cx, |hovered, cx| {
                    *hovered = *value;
                    cx.notify();
                })
            })
    }
}
