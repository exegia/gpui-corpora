use super::*;
use gpui_kit::{
    base::{StyledExt, Transition, transition},
    component::{
        Theme,
        resizable::{h_resizable, resizable_panel},
    },
};

use crate::shell::tab_workspace::{ToggleInspector, ToggleSidebar};

pub(super) fn route_title(path: &str) -> &'static str {
    match path.split('/').nth(1).unwrap_or_default() {
        "explore" => "Explore",
        "projects" => "Projects",
        "settings" => "Settings",
        "profile" => "Profile",
        "notifications" => "Notifications",
        "diagnostics" => "Diagnostics",
        "about" => "About",
        _ => "Home",
    }
}

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

pub(super) fn layout(
    workspace: WeakEntity<Workspace>,
    body: impl IntoElement,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let Some(owner) = workspace.upgrade() else {
        return body.into_any_element();
    };
    let tab = owner
        .read(cx)
        .tabs
        .active()
        .expect("workspace has a tab")
        .clone();
    let state = &tab.state;
    let motion = Theme::global(cx).motion_tokens();
    let policy = Transition::new(motion.duration_normal).easing(motion.easing_move.clone());
    let sidebar = transition(
        format!("{}-sidebar", tab.id),
        if state.sidebar_visible { 1.0_f32 } else { 0.0 },
        policy.clone(),
        window,
        cx,
    );
    let inspector = transition(
        format!("{}-inspector", tab.id),
        if state.inspector_visible {
            1.0_f32
        } else {
            0.0
        },
        policy,
        window,
        cx,
    );
    let rem = window.rem_size();
    let dark = cx.theme().is_dark();
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
                Button::new(format!("sidebar-{label}"), label)
                    .icon(icon)
                    .variant(if selected == path {
                        ButtonVariant::Subtle
                    } else {
                        ButtonVariant::Ghost
                    })
                    .on_click(move |_, window, cx| use_navigate(window, cx).push(path))
            }),
        );
    let inspector_content = div()
        .id(format!("{}-inspector-content", tab.id))
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&state.inspector_scroll)
        .p_3()
        .flex()
        .flex_col()
        .gap_3()
        .child(div().text_sm().font_semibold().child("Inspector"))
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().colors.fg_muted)
                .child("No selection"),
        );
    let split = h_resizable(format!("{}-split", tab.id))
        .with_state(&state.split)
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
                .child(body),
        )
        .child(
            resizable_panel()
                .visible(inspector > 0.0)
                .flex_none()
                .size(inspector_width)
                .size_range(range)
                .pl(rems(0.5 * inspector))
                .overflow_hidden()
                .child(crate::shell::dev_target(
                    "tab-inspector",
                    "Tab inspector",
                    corpora_ui::ui::layout::inspector(dark, inspector_content),
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
                        corpora_ui::ui::layout::sidebar(dark, sidebar_content),
                    )),
            )
        })
        .child(div().flex_1().min_w_0().h_full().child(split))
        .into_any_element()
}
