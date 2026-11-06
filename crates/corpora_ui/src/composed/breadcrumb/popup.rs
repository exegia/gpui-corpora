use std::time::Duration;

use gpui::{
    AnyElement, App, Context, ElementId, Entity, FocusHandle, Focusable, IntoElement, RenderOnce,
    Subscription, Window, div, prelude::*,
};
use gpui_kit::{
    base::{
        Popup,
        actions::Cancel,
        animation::cubic_bezier,
        motion::{Presence, Transition},
    },
    component::popover::PopoverState,
};

use super::BreadcrumbTrigger;
use crate::ely::theme::{ActiveTheme, Radius};

mod offset;
use offset::PaintOffset;

const MOTION_DURATION: Duration = Duration::from_millis(240);

/// A start-aligned breadcrumb surface; content owns its data and selection.
#[derive(IntoElement)]
pub struct BreadcrumbPopup {
    id: ElementId,
    trigger: BreadcrumbTrigger,
    content: Box<dyn Fn(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> AnyElement>,
}

impl BreadcrumbPopup {
    pub fn new<E: IntoElement>(
        id: impl Into<ElementId>,
        trigger: BreadcrumbTrigger,
        content: impl Fn(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> E + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            trigger,
            content: Box::new(move |state, window, cx| {
                content(state, window, cx).into_any_element()
            }),
        }
    }
}

// Closing content stays painted, so redirect focus until presence unmounts it.
struct ExitFocus {
    target: Option<FocusHandle>,
    _subscription: Subscription,
}

fn toggle(state: &Entity<PopoverState>, window: &mut Window, cx: &mut App) {
    state.update(cx, |state, cx| {
        if state.is_open() {
            state.dismiss(window, cx);
        } else {
            state.show(window, cx);
        }
    });
}

impl RenderOnce for BreadcrumbPopup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.trigger.is_disabled() {
            return self.trigger.into_any_element();
        }
        let label = self.trigger.label();
        let state = window.use_keyed_state((self.id.clone(), "state"), cx, |_, cx| {
            PopoverState::new(false, cx)
        });
        let open = state.read(cx).is_open();
        let focus = state.read(cx).focus_handle(cx);
        let weak = state.downgrade();
        let exit_focus =
            window.use_keyed_state((self.id.clone(), "exit-focus"), cx, |window, cx| {
                ExitFocus {
                    target: None,
                    _subscription: cx.on_focus_in(
                        &focus,
                        window,
                        move |guard: &mut ExitFocus, window, cx| {
                            if weak.upgrade().is_none_or(|state| !state.read(cx).is_open()) {
                                if let Some(target) = &guard.target {
                                    target.focus(window, cx);
                                }
                            }
                        },
                    ),
                }
            });
        if !open && !focus.contains_focused(window, cx) {
            let target = window.focused(cx);
            exit_focus.update(cx, |guard, _| guard.target = target);
        }
        let presence = Presence::new((self.id.clone(), "presence"), open)
            .transition(Transition::new(MOTION_DURATION).ease(cubic_bezier(0.2, 0.0, 0.0, 1.0)))
            .sample(window, cx);
        let parent = window.current_view();
        let activate = state.clone();
        let mut popup = Popup::new(
            self.id,
            self.trigger.popup(open, move |_, window, cx| {
                toggle(&activate, window, cx);
                cx.notify(parent);
            }),
        )
        .offset(window.rem_size() * 0.25)
        .key_context("Popover");
        if !presence.should_render() {
            return popup.into_any_element();
        }
        let body = state.update(cx, |state, cx| (self.content)(state, window, cx));
        let theme = cx.theme();
        let width = (window.viewport_size().width - window.rem_size() * 2.0).max(window.rem_size());
        let progress = presence.progress;
        let rem = window.rem_size();
        // GPUI fades primitives separately; delay shadow ink to avoid bleed through the surface.
        let shadow = gpui::BoxShadow {
            color: theme.colors.shadow.opacity(0.12 * progress.powi(3)),
            offset: gpui::point(gpui::Pixels::ZERO, rem * 0.375),
            blur_radius: rem * 0.5,
            spread_radius: gpui::Pixels::ZERO,
            inset: false,
        };
        let cancel = state.clone();
        let outside = state.clone();
        let surface = div()
            .id("breadcrumb-popup-surface")
            .relative()
            .role(gpui::Role::Dialog)
            .aria_label(label)
            .tab_group()
            .track_focus(&focus)
            .key_context("Popover")
            .p_4()
            .max_w(width)
            .max_h((window.viewport_size().height - rem * 2.0).max(rem))
            .overflow_y_scroll()
            .min_w_0()
            .bg(theme.colors.bg)
            .text_color(theme.colors.fg)
            .rounded(theme.radius(Radius::Lg))
            .border_1()
            .border_color(theme.colors.border)
            .shadow(vec![shadow])
            .opacity(progress)
            .capture_any_mouse_down(move |_, window, cx| {
                if !open {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .capture_any_mouse_up(move |_, window, cx| {
                if !open {
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .on_action(move |_: &Cancel, window, cx| {
                cancel.update(cx, |state, cx| state.dismiss(window, cx));
                cx.notify(parent);
            })
            .when(open, |surface| {
                surface.on_mouse_down_out(move |_, window, cx| {
                    outside.update(cx, |state, cx| state.dismiss(window, cx));
                    cx.notify(parent);
                })
            })
            .child(body);
        popup = popup.content(PaintOffset::new(surface, -rem * 0.5 * (1.0 - progress)));
        popup.into_any_element()
    }
}
