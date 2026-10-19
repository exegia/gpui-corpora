#[allow(unused_imports)]
use super::*;

/// Built inside the root route so location and selected navigation agree in this frame.
pub fn frame(
    workspace: WeakEntity<Workspace>,
    outlet: Outlet,
    _: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let path = use_location(cx).pathname.to_string();
    let selected = routes::section(&path);
    let (sidebar_visible, loading, error_view) = workspace
        .update(cx, |state, cx| {
            (state.sidebar_visible, state.loading, state.error_view(cx))
        })
        .unwrap_or((true, false, None));
    {
        let workspace = workspace.clone();
        let path = path.clone();
        cx.defer(move |cx| {
            workspace
                .update(cx, |state, cx| state.path_rendered(&path, cx))
                .ok();
        });
    }
    let sidebar_owner = workspace.clone();
    let dock_owner = workspace.clone();
    let palette_owner = workspace.clone();
    // Ely TitleBar already uses its drag_region (the WindowDragRegion equivalent).
    let titlebar = TitleBar::new("titlebar")
        .child(
            pages::nav_button("workspace-home", "Corpora", navigation::HOME)
                .icon(IconName::BookOpen),
        )
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
        .child(
            TabBar::new("workspace-tabs")
                .tab(WindowTab::new(navigation::HOME, "Home").icon(IconName::House))
                .tab(WindowTab::new(navigation::EXPLORE, "Explore").icon(IconName::Compass))
                .tab(WindowTab::new(navigation::PROJECTS, "Projects").icon(IconName::Folder))
                .selected(selected)
                .on_select(|path, window, cx| use_navigate(window, cx).push(path.clone())),
        )
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
            IconButton::new("toggle-sidebar", IconName::PanelLeft)
                .tooltip("Toggle sidebar")
                .on_click(move |_, _, cx| {
                    sidebar_owner
                        .update(cx, |state, cx| state.toggle_sidebar(cx))
                        .ok();
                }),
        )
        .action(
            IconButton::new("toggle-dock", IconName::PanelBottom)
                .tooltip("Toggle activity dock")
                .on_click(move |_, _, cx| {
                    dock_owner
                        .update(cx, |state, cx| {
                            state.dock_visible = !state.dock_visible;
                            cx.notify();
                        })
                        .ok();
                }),
        )
        .action(
            IconButton::new("settings", IconName::Settings)
                .tooltip("Settings")
                .on_click(|_, window, cx| use_navigate(window, cx).push(navigation::SETTINGS)),
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
        .item(ActivityItem::new(
            navigation::NOTIFICATIONS,
            IconName::Bell,
            "Notifications",
        ))
        .footer(ActivityItem::new(
            navigation::SETTINGS,
            IconName::Settings,
            "Settings",
        ))
        .footer(ActivityItem::new(
            navigation::PROFILE,
            IconName::User,
            "Profile",
        ))
        .selected(selected)
        .on_select(|path, window, cx| use_navigate(window, cx).push(path.clone()));
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
    div().size_full().flex().flex_col().child(titlebar).child(
        div()
            .flex()
            .flex_1()
            .min_h_0()
            .child(activity)
            .when(sidebar_visible, |row| row.child(sidebar(&path)))
            .child(
                div()
                    .id("route-content")
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .overflow_y_scroll()
                    .child(content),
            ),
    )
}
