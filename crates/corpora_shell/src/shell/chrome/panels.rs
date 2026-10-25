use super::*;
use corpora_ui::ui::layout;
use gpui_kit::{
    base::{ResizeHandleRenderer, StyledExt, Transition, transition},
    component::{
        Theme,
        resizable::{h_resizable, resizable_panel, resize_handle_appearance, v_resizable},
    },
};

use crate::shell::tab_workspace::{ToggleInspector, ToggleSidebar};

pub(super) fn toggle(inspector: bool, visible: bool) -> IconButton {
    let (id, icon, name, shortcut) = if inspector {
        (
            "toggle-inspector",
            IconName::PanelRight,
            "inspector",
            "secondary-shift-b",
        )
    } else {
        (
            "toggle-sidebar",
            IconName::PanelLeft,
            "sidebar",
            "secondary-b",
        )
    };
    IconButton::new(id, icon)
        .variant(if visible {
            ButtonVariant::Subtle
        } else {
            ButtonVariant::Ghost
        })
        .tooltip(format!(
            "{} {name} ({})",
            if visible { "Hide" } else { "Show" },
            crate::app::keys::label(shortcut)
        ))
        .on_click(move |_, window, cx| {
            if inspector {
                window.dispatch_action(Box::new(ToggleInspector), cx);
            } else {
                window.dispatch_action(Box::new(ToggleSidebar), cx);
            }
        })
}

/// Half the gap between split panes. Each side pads by this much so the
/// splitter, which sits on the panel boundary, lands in the gap's center.
const HALF_GAP: f32 = 0.25;

/// Kit's splitter with its resting hairline hidden; the pill still shows on hover and drag.
fn splitter() -> ResizeHandleRenderer {
    let kit = resize_handle_appearance();
    std::rc::Rc::new(move |handle, window, cx| {
        let mut line = kit(handle, window, cx)?;
        if let Some(line) = line.downcast_mut::<gpui::Div>() {
            line.style()
                .refine(&StyleRefinement::default().bg(gpui::transparent_black()));
        }
        Some(line)
    })
}

/// A pane's title over a muted line, for panes with nothing to show yet.
fn placeholder(title: &'static str, detail: &'static str, cx: &App) -> gpui::Div {
    div()
        .size_full()
        .p_3()
        .flex()
        .flex_col()
        .gap_3()
        .child(div().text_sm().font_semibold().child(title))
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().colors.fg_muted)
                .child(detail),
        )
}

/// Lays the active view's panes around `content`, as its route's `Panes` allow.
pub(super) fn layout(
    workspace: WeakEntity<Workspace>,
    content: impl IntoElement,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let dark = cx.theme().is_dark();
    let Some(owner) = workspace.upgrade() else {
        return layout::main(dark, content)
            .flush_right(true)
            .into_any_element();
    };
    let Some(tab) = owner.read(cx).tabs.active().cloned() else {
        return div().flex_1().min_w_0().h_full().into_any_element();
    };
    let state = &tab.state;
    let panes = routes::panes(&tab.path);
    let motion = Theme::global(cx).motion_tokens();
    let policy = Transition::new(motion.duration_normal).easing(motion.easing_move.clone());
    let sidebar = transition(
        format!("{}-sidebar", tab.id),
        if panes.sidebar && state.panels.sidebar_visible {
            1.0_f32
        } else {
            0.0
        },
        policy.clone(),
        window,
        cx,
    );
    let inspector = transition(
        format!("{}-inspector", tab.id),
        if panes.inspector && state.panels.inspector_visible {
            1.0_f32
        } else {
            0.0
        },
        policy,
        window,
        cx,
    );
    let rem = window.rem_size();
    // Reserve the central viewport even when the window is narrow or zoomed in.
    let sidebar_width = rems(16.0)
        .to_pixels(rem)
        .min(window.viewport_size().width * 0.24);
    let max_inspector = rems(32.0)
        .to_pixels(rem)
        .min(window.viewport_size().width * 0.32);
    let min_inspector = rems(12.0).to_pixels(rem).min(max_inspector);
    let inspector_width = state
        .inspector_width
        .to_pixels(rem)
        .clamp(min_inspector, max_inspector);
    let range = if inspector < 1.0 || (sidebar > 0.0 && sidebar < 1.0) {
        let width = inspector_width * inspector;
        width..width
    } else {
        min_inspector..max_inspector
    };
    let tab_id = tab.id.clone();
    let selected = if tab.path.starts_with(EMPTY_TAB_PREFIX) {
        ""
    } else {
        routes::section(&tab.path)
    };
    let sidebar_content = div()
        .id(format!("{}-sidebar-content", tab.id))
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&state.sidebar_scroll)
        .p_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_sm().font_semibold().mb_2().child("Workspace"))
        .children(
            [
                ("/", "Home", IconName::House),
                ("/explore", "Explore", IconName::Compass),
                ("/projects", "Projects", IconName::Folder),
            ]
            .into_iter()
            .map(|(path, label, icon)| {
                let owner = workspace.clone();
                Button::new(format!("sidebar-{label}"), label)
                    .icon(icon)
                    .variant(if selected == path {
                        ButtonVariant::Subtle
                    } else {
                        ButtonVariant::Ghost
                    })
                    .on_click(move |_, _, cx| {
                        owner.update(cx, |state, cx| state.navigate(path, cx)).ok();
                    })
            }),
        );
    let inspector_content = div()
        .id(format!("{}-inspector-content", tab.id))
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&state.inspector_scroll)
        .child(placeholder("Inspector", "No selection", cx));
    // Only panes that meet the window's right edge keep square right corners.
    let flush = inspector == 0.0;
    let primary = layout::main(dark, content).flush_right(flush && !panes.split);
    let top = if panes.split {
        h_resizable(format!("{}-content-split", tab.id))
            .with_state(&state.content_split)
            .with_handle_appearance(splitter())
            .child(resizable_panel().pr(rems(HALF_GAP)).child(primary))
            .child(
                resizable_panel()
                    .pl(rems(HALF_GAP))
                    .child(crate::shell::dev_target(
                        "tab-split-pane",
                        "Tab split pane",
                        layout::main(dark, placeholder("Split view", "Nothing open", cx))
                            .flush_right(flush),
                    )),
            )
            .into_any_element()
    } else {
        primary.into_any_element()
    };
    let center = if panes.dock {
        v_resizable(format!("{}-dock-split", tab.id))
            .with_state(&state.dock_split)
            .with_handle_appearance(splitter())
            .child(resizable_panel().pb(rems(HALF_GAP)).child(top))
            .child(
                resizable_panel()
                    .pt(rems(HALF_GAP))
                    .child(crate::shell::dev_target(
                        "tab-bottom-panel",
                        "Tab bottom panel",
                        layout::main(dark, placeholder("Activity", "No workspace events", cx))
                            .flush_right(flush),
                    )),
            )
            .into_any_element()
    } else {
        top
    };
    let split = h_resizable(format!("{}-split", tab.id))
        .with_state(&state.split)
        .with_handle_appearance(splitter())
        .on_resize(move |split, window, cx| {
            if let Some(width) = split.read(cx).sizes().get(1).copied() {
                workspace
                    .update(cx, |owner, cx| {
                        if let Some(tab) = owner.tabs.active_mut().filter(|tab| tab.id == tab_id) {
                            tab.state.inspector_width = rems(width / window.rem_size());
                            cx.notify();
                        }
                    })
                    .ok();
            }
        })
        .child(
            resizable_panel()
                .size_range(gpui::px(0.0)..gpui::Pixels::MAX)
                .pr(rems(HALF_GAP * inspector))
                .child(center),
        )
        .child(
            resizable_panel()
                .visible(inspector > 0.0)
                .flex_none()
                .size(inspector_width)
                .size_range(range)
                .pl(rems(HALF_GAP * inspector))
                .overflow_hidden()
                .child(crate::shell::dev_target(
                    "tab-inspector",
                    "Tab inspector",
                    layout::inspector(dark, inspector_content),
                )),
        );
    div()
        .id(format!("{}-layout", tab.id))
        .size_full()
        .flex()
        .min_w_0()
        .when(sidebar > 0.0, |row| {
            row.child(
                div()
                    .w(sidebar_width * sidebar)
                    .h_full()
                    .flex_shrink_0()
                    .overflow_hidden()
                    .pr(rems(0.5 * sidebar))
                    .child(crate::shell::dev_target(
                        "tab-sidebar",
                        "Tab sidebar",
                        layout::sidebar(dark, sidebar_content),
                    )),
            )
        })
        .child(div().flex_1().min_w_0().h_full().child(split))
        .into_any_element()
}
