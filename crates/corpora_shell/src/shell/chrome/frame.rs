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
    let selected = if path.starts_with(EMPTY_TAB_PREFIX) {
        ""
    } else {
        routes::section(&path)
    };
    let (sidebar_visible, loading, error_view) = workspace
        .update(cx, |state, cx| {
            (state.sidebar_visible, state.loading, state.error_view(cx))
        })
        .unwrap_or((true, false, None));
    {
        let workspace = workspace.clone();
        let path = path.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |state, cx| state.path_rendered(&path, window, cx))
                .ok();
        });
    }
    let tab_owner = workspace.clone();
    let palette_owner = workspace.clone();
    let tabs = tabs::tabs(workspace.clone(), window, cx);
    let notifications = notifications::notifications(workspace.clone(), cx);
    // Ely TitleBar already uses its drag_region (the WindowDragRegion equivalent).
    let titlebar = TitleBar::new("titlebar")
        .child(
            IconButton::new("back", IconName::ArrowLeft)
                .tooltip("Back")
                .on_click(|_, window, cx| use_navigate(window, cx).back()),
        )
        .child(
            IconButton::new("forward", IconName::ArrowRight)
                .tooltip("Forward")
                .on_click(|_, window, cx| use_navigate(window, cx).forward()),
        )
        .child(tabs)
        .action(
            IconButton::new("new-workspace-tab", IconName::Plus)
                .tooltip("New tab")
                .on_click(move |_, _, cx| {
                    tab_owner
                        .update(cx, |state, cx| {
                            let path = state.tabs.new_empty();
                            state.navigate(path, cx);
                        })
                        .ok();
                }),
        )
        .action(notifications)
        .action(
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
        )
        .action(
            IconButton::new("settings", IconName::Settings)
                .tooltip("Settings")
                .on_click(|_, window, cx| use_navigate(window, cx).push(navigation::SETTINGS)),
        );
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
    let activity = ActivityBar::new("activity-bar")
        .item(ActivityItem::new(navigation::HOME, IconName::House, "Home"))
        .item(ActivityItem::new(
            navigation::EXPLORE,
            IconName::Compass,
            "Explore",
        ))
        .item(ActivityItem::new(
            navigation::PROJECTS,
            IconName::Folder,
            "Projects",
        ))
        .footer(ActivityItem::new(
            navigation::PROFILE,
            IconName::User,
            "Profile",
        ))
        .selected(selected)
        .on_select(|path, window, cx| use_navigate(window, cx).push(path.clone()));
    let activity = style_chrome(
        activity,
        StyleRefinement::default()
            .border_r_0()
            .bg(gpui::transparent_black()),
        window,
        cx,
    );
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
            None => div().p_8().child(outlet).into_any_element(),
        }
    };
    let body = style_chrome(
        Well::new().flex_1().min_w_0().h_full().child(
            div()
                .id("route-content")
                .size_full()
                .overflow_y_scroll()
                .child(crate::shell::dev_target(
                    "route-page",
                    "Current page",
                    content,
                )),
        ),
        workspace_surface::style(cx.theme().is_dark()),
        window,
        cx,
    );
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
                .when(sidebar_visible, |row| row.child(sidebar(&path)))
                .child(body),
        )
}
