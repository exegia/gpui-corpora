#[allow(unused_imports)]
use super::*;

/// Built inside the root route so location and selected navigation agree in this frame.
pub fn frame(
    workspace: WeakEntity<Workspace>,
    outlet: Outlet,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let path = use_location(cx).pathname.to_string();
    let has_view = workspace
        .upgrade()
        .is_some_and(|owner| owner.read(cx).tabs.active().is_some());
    let selected = if !has_view || path.starts_with(EMPTY_TAB_PREFIX) {
        ""
    } else {
        routes::section(&path)
    };
    let (loading, error_view) = workspace
        .update(cx, |state, cx| (state.loading, state.error_view(cx)))
        .unwrap_or((false, None));
    {
        let workspace = workspace.clone();
        let path = path.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |state, cx| state.path_rendered(&path, window, cx))
                .ok();
        });
    }
    let palette_owner = workspace.clone();
    let tabs = tabs::tabs(workspace.clone(), window, cx);
    let notifications = notifications::notifications(workspace.clone(), cx);
    // Ely TitleBar already uses its drag_region (the WindowDragRegion equivalent).
    let (panes, sidebar_visible, inspector_visible, scroll) = workspace
        .upgrade()
        .and_then(|owner| {
            let owner = owner.read(cx);
            let tab = owner.tabs.active()?;
            Some((
                routes::panes(&tab.path),
                tab.state.sidebar_visible,
                tab.state.inspector_visible,
                tab.state.scroll.clone(),
            ))
        })
        .unwrap_or_default();
    let settings_owner = workspace.clone();
    let titlebar = TitleBar::new("titlebar")
        .child(brand::brand(cx))
        .child(tabs)
        .action(round(
            panels::toggle(false, sidebar_visible).disabled(!panes.sidebar),
            window,
            cx,
        ))
        .action(round(
            panels::toggle(true, inspector_visible).disabled(!panes.inspector),
            window,
            cx,
        ))
        .action(notifications)
        .action(round(
            IconButton::new("open-palette", IconName::Search)
                .tooltip(format!(
                    "Command palette ({})",
                    crate::app::keys::label(crate::app::keys::TOGGLE_SEARCH)
                ))
                .on_click(move |_, _, cx| {
                    palette_owner
                        .update(cx, |state, cx| state.toggle_palette(cx))
                        .ok();
                }),
            window,
            cx,
        ))
        .action(round(
            IconButton::new("settings", IconName::Settings)
                .tooltip("Settings")
                .on_click(move |_, _, cx| {
                    settings_owner
                        .update(cx, |state, cx| state.navigate(navigation::SETTINGS, cx))
                        .ok();
                }),
            window,
            cx,
        ));
    let titlebar = style_chrome(
        titlebar,
        StyleRefinement::default()
            .h(cx.theme().titlebar_height() + TITLEBAR_PADDING * 2.0)
            .px_1()
            .bg(gpui::transparent_black())
            .border_b_0(),
        window,
        cx,
    );
    let activity = activity::activity(selected, workspace.clone(), cx);
    let content = if loading {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .child(Skeleton::new("loading-heading").w_64().h_8())
            .child(Skeleton::new("loading-description").w_96().h_4())
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(Skeleton::new("loading-card-one").flex_1().h_32())
                    .child(Skeleton::new("loading-card-two").flex_1().h_32())
                    .child(Skeleton::new("loading-card-three").flex_1().h_32()),
            )
            .into_any_element()
    } else {
        match error_view {
            Some(view) => div().p_8().child(view).into_any_element(),
            None => div()
                .p_8()
                .when(matches!(path.as_str(), "/explore" | "/projects"), |page| {
                    page.h_full()
                })
                .child(outlet)
                .into_any_element(),
        }
    };
    let body = div()
        .id("route-content")
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&scroll)
        .child(crate::shell::dev_target(
            "route-page",
            "Current page",
            content,
        ));
    let body = panels::layout(workspace, body, window, cx);
    div()
        .border_0()
        .size_full()
        .flex()
        .flex_col()
        .child(crate::shell::dev_target(
            "titlebar",
            "Title bar and window tabs",
            titlebar,
        ))
        .child(
            div()
                .flex()
                .flex_1()
                .min_h_0()
                .child(crate::shell::dev_target(
                    "activity-bar",
                    "Activity bar",
                    activity,
                ))
                .child(body),
        )
}
