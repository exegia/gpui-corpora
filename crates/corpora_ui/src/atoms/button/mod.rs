//! Corpora Button variants and composition backed by native Kit activation.

mod appearance;
mod semantics;
mod size;

pub use appearance::{ButtonShape, ButtonVariant};
pub use size::ButtonSize;

use std::time::Duration;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, IntoElement, MouseButton, ParentElement, RenderOnce, Role, SharedString, StyleRefinement, Styled, Window, div, prelude::*, relative, rems,
};
use gpui_kit::{
    base::{
        Button as NativeButton, StyledExt, TestSupportExt,
        animation::cubic_bezier,
        motion::{Transition, transition},
    },
    component::{Sizable, Size, spinner::Spinner},
};

use crate::{
    ely::{
        primitives::IconName,
        theme::{ActiveTheme, Radius},
    }, ui::material::GlassSurfaceExt as _,
};

#[derive(Default)]
struct Interaction {
    hovered: bool,
    pressed: bool,
}

/// A labeled native button. Supplied children replace the default text content.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    loading: bool,
    disabled: bool,
    focus: Option<FocusHandle>,
    href: Option<SharedString>,
    icon: Option<IconName>,
    children: Vec<AnyElement>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            shape: ButtonShape::default(),
            loading: false,
            disabled: false,
            focus: None,
            href: None,
            icon: None,
            children: Vec::new(),
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Sizes a leading icon with the control; icon sizes omit the visible label.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn track_focus(mut self, focus: &FocusHandle) -> Self {
        self.focus = Some(focus.clone());
        self
    }

    /// Opens the URL through the application after activation and any click handler.
    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn timing(milliseconds: u64) -> Transition {
    Transition::new(Duration::from_millis(milliseconds)).ease(cubic_bezier(0.22, 1., 0.36, 1.))
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let inert = self.disabled || self.loading;
        let accessibility_label = self.label.clone();
        let accessibility_role = if self.href.is_some() {
            Role::Link
        } else {
            Role::Button
        };
        let state = window.use_keyed_state((self.id.clone(), "interaction"), cx, |_, _| {
            Interaction::default()
        });
        let focus = self.focus.unwrap_or_else(|| {
            window
                .use_keyed_state((self.id.clone(), "focus"), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone()
        });
        let hovered = !inert && state.read(cx).hovered;
        let pressed = !inert && state.read(cx).pressed && (hovered || focus.is_focused(window));
        let compact = window.viewport_size().width >= window.rem_size() * 40.;
        let metrics = self.size.metrics(compact);
        let inline_link = self.variant == ButtonVariant::Link && !self.size.is_icon();
        let colors = transition(
            (self.id.clone(), "colors"),
            self.variant.colors(cx.theme(), hovered, pressed),
            timing(150),
            window,
            cx,
        );
        let glass = self
                    .variant
                    .glass_spec(cx.theme(), colors.glass_tint, colors.engage);
        let height = transition(
            (self.id.clone(), "height"),
            metrics.height / 16.,
            timing(300),
            window,
            cx,
        );
        let padding = transition(
            (self.id.clone(), "padding"),
            if inline_link {
                0.
            } else {
                metrics.padding / 16.
            },
            timing(300),
            window,
            cx,
        );
        let loading = transition(
            (self.id.clone(), "loading"),
            if self.loading { 1_f32 } else { 0. },
            timing(300),
            window,
            cx,
        );
        let scale = transition(
            (self.id.clone(), "press"),
            if pressed && !cx.reduce_motion() {
                0.97_f32
            } else {
                1.
            },
            timing(150),
            window,
            cx,
        );
        let shape = self.shape.is_default();
        let radius = cx
            .theme()
            .radius(Radius::Xl);
        let rem = window.rem_size();
        let icon_scale = transition(
            (self.id.clone(), "icon-hover"),
            if hovered && !self.size.is_icon() && !cx.reduce_motion() {
                1.01_f32
            } else {
                1.
            },
            timing(150),
            window,
            cx,
        );
        let mut content = Vec::new();
        if let Some(icon) = self.icon {
            content.push(
                gpui::svg()
                    .path(icon.path())
                    .size(rems(metrics.icon / 16.))
                    .flex_none()
                    .mx(rems(-0.5 / 16.))
                    .text_color(colors.foreground.opacity(0.8))
                    .with_transformation(gpui::Transformation::scale(gpui::size(
                        icon_scale * scale,
                        icon_scale * scale,
                    )))
                    .into_any_element(),
            );
        }
        if self.children.is_empty() {
            if self.icon.is_none() || !self.size.is_icon() {
                content.push(self.label.clone().into_any_element());
            }
        } else {
            content.extend(self.children);
        }
        let spinner = || {
            Spinner::new()
                .with_size(Size::Size(rem * (metrics.icon / 16.)))
                .color(colors.foreground)
        };

        let mut button = NativeButton::new(self.id.clone())
            .accessibility_label(self.label)
            .disabled(inert)
            .tab_stop(!inert)
            .track_focus(&focus)
            .relative()
            .flex_none()
            .h(rems(height))
            .px(rems(padding))
            .gap(rems(metrics.gap / 16.))
            .when(self.variant == ButtonVariant::Ghost, |button| {
                button.bg(colors.foreground.opacity(colors.highlight))
            })
            .rounded(radius)
            // .border_1()
            // .border_color(colors.border)
            .when(!inline_link, |button| {
                button.border_1().border_color(colors.border.alpha(0.))
            })
            .text_size(rems(metrics.font / 16.))
            .font_weight(gpui::FontWeight::MEDIUM)
            .text_color(colors.foreground)
            .whitespace_nowrap()
            .line_height(relative(1.5))
            .when(inline_link, |button| button.h_auto())
            .when(self.size.is_icon(), |button| button.w(rems(height)).p_0())
            .when(inert, |button| {
                button.opacity(if self.variant == ButtonVariant::Default {
                    0.5
                } else {
                    0.64
                })
            })
            .when(!inert, |button| button.cursor_pointer())
            .when(self.href.is_some(), |button| button.role(Role::Link))
            .when(!self.size.is_icon(), |button| {
                button.child(
                    div()
                        .id((self.id.clone(), "loading-slot"))
                        .test_support()
                        .flex()
                        .flex_none()
                        .items_center()
                        .overflow_hidden()
                        .w(rems(metrics.icon / 16. * loading))
                        .ml(rems(-metrics.gap / 16. * (1. - loading)))
                        .opacity(loading)
                        .when(loading > 0., |slot| slot.child(spinner())),
                )
            })
            .child(
                div()
                    .id((self.id.clone(), "content"))
                    .test_support()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(rems(metrics.gap / 16.))
                    .when(self.size.is_icon() && self.loading, |content| {
                        content.opacity(0.)
                    })
                    .when(
                        self.variant == ButtonVariant::Link && (hovered || pressed),
                        |content| content.underline(),
                    )
                    .children(content),
            )


            .when(self.size.is_icon() && self.loading, |button| {
                button.child(div().absolute().child(spinner()))
            });

        if !inert {
            let down = state.clone();
            let up = state.clone();
            let out = state.clone();
            let hover = state.clone();
            let key_down = state.clone();
            let key_up = state;
            let mouse_focus = focus;
            button = button
                .on_hover(move |value, _, cx| {
                    hover.update(cx, |state, cx| {
                        if state.hovered != *value {
                            state.hovered = *value;
                            if !value {
                                state.pressed = false;
                            }
                            cx.notify();
                        }
                    })
                })
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    mouse_focus.focus(window, cx);
                    down.update(cx, |state, cx| {
                        if !state.pressed {
                            state.pressed = true;
                            cx.notify();
                        }
                    });
                })
                .on_mouse_up(MouseButton::Left, move |_, _, cx| {
                    up.update(cx, |state, cx| {
                        if state.pressed {
                            state.pressed = false;
                            cx.notify();
                        }
                    })
                })
                .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                    out.update(cx, |state, cx| {
                        if state.pressed {
                            state.pressed = false;
                            cx.notify();
                        }
                    })
                })
                .capture_key_down(move |event, _, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        key_down.update(cx, |state, cx| {
                            if !state.pressed {
                                state.pressed = true;
                                cx.notify();
                            }
                        });
                    }
                })
                .capture_key_up(move |event, _, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        key_up.update(cx, |state, cx| {
                            if state.pressed {
                                state.pressed = false;
                                cx.notify();
                            }
                        });
                    }
                });
        }
        let click = self.on_click;
        let href = self.href;
        button = button.on_click(move |event, window, cx| {
            if let Some(handler) = &click {
                handler(event, window, cx);
            }
            if let Some(href) = &href {
                cx.open_url(href);
            }
        });
        // Caller overrides win over the variant/size defaults set above.
        button = button
         .refine_style(&self.style);
        let button = match glass {
            Some(spec) => button.glass_with(spec).scale(scale).into_any_element(),
            None => button.into_any_element(),
        };
        semantics::ButtonSemantics::new(
            button,
            accessibility_label,
            accessibility_role,
            inert,
            self.loading,
        )
    }
}

#[cfg(test)]
#[path = "button.test.rs"]
mod tests;
