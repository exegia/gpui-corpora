use super::*;
use gpui::{AppContext, Bounds, Pixels, TestAppContext, WindowBounds, WindowOptions, px, size};
use gpui_kit::test::TestWindowExt;

fn setup(cx: &mut TestAppContext, reduced: bool) -> (gpui::AnyWindowHandle, Entity<Workspace>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        gpui_router::init(cx);
        cx.set_reduce_motion(reduced);
        cx.set_global(events::AppEventQueue::default());
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(1200.), px(800.)),
                })),
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| {
                    let mut view = Workspace::new(window, cx);
                    view.loading = false;
                    view
                })
            },
        )
        .unwrap()
    })
}

#[gpui::test]
fn direct_router_navigation_leaves_the_tab_layout_and_keeps_tab_state(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| view.new_tab(window, cx));
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
    })
    .unwrap();
    cx.update_window(handle, |_, window, cx| {
        let tab = view.read(cx).tabs.active().unwrap().path.clone();
        assert!(tab.starts_with(EMPTY_TAB_PREFIX));
        gpui_router::use_navigate(window, cx).push("/explore");
        window.render_frame(cx);
        let state = view.read(cx);
        assert_eq!(state.tabs.active().unwrap().path, "/explore");
        assert!(!routes::panes(&state.tabs.active().unwrap().path).split);
        assert_eq!(state.tabs.entries().last().unwrap().path, tab);
        gpui_router::use_navigate(window, cx).push("/ui/button");
        window.render_frame(cx);
        assert_eq!(view.read(cx).tabs.active_id(), Some("/ui"));
        assert_eq!(view.read(cx).tabs.active().unwrap().path, "/ui/button");
    })
    .unwrap();
}

fn inspector_width(window: &Window) -> Pixels {
    window
        .try_find("workspace-inspector")
        .map(|element| element.bounds().size.width)
        .unwrap_or(px(0.))
}

#[gpui::test]
fn inspector_reverses_continuously_and_keeps_its_width_while_sidebar_moves(
    cx: &mut TestAppContext,
) {
    let (handle, view) = setup(cx, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| view.navigate("/projects", cx));
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(0.));
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(80));
    let middle = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let middle = inspector_width(window);
            assert!(middle > px(0.) && middle < px(320.));
            view.update(cx, |view, cx| {
                view.tabs
                    .active_mut()
                    .unwrap()
                    .state
                    .panels
                    .inspector_visible = false;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(inspector_width(window), middle);
            middle
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(400));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(0.));
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(400));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let open = inspector_width(window);
        assert!(open > middle);
        assert_eq!(open, px(320.));
        view.update(cx, |view, cx| {
            view.tabs.active_mut().unwrap().state.panels.sidebar_visible = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), open);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(400));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(320.));
    })
    .unwrap();
}

#[gpui::test]
fn queued_navigation_keeps_outgoing_state_until_the_latest_route_commits(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| view.new_tab(window, cx));
        assert_eq!(view.read(cx).tabs.active().unwrap().path, "/");
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(
            view.read(cx)
                .tabs
                .active()
                .unwrap()
                .path
                .starts_with(EMPTY_TAB_PREFIX)
        );
        view.update(cx, |view, cx| {
            view.navigate("/projects", cx);
            view.navigate("/explore", cx);
        });
        assert!(
            view.read(cx)
                .tabs
                .active()
                .unwrap()
                .path
                .starts_with(EMPTY_TAB_PREFIX)
        );
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).tabs.active().unwrap().path, "/explore");
        assert_eq!(gpui_router::use_location(cx).pathname.as_ref(), "/explore");
        assert!(window.try_find("workspace-sidebar").is_none());
        assert!(window.try_find("workspace-inspector").is_none());
    })
    .unwrap();
}

#[gpui::test]
fn inspector_drag_and_reduced_motion_preserve_the_remembered_width(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        gpui_router::use_navigate(window, cx).push("/projects");
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(320.));
        let bounds = window.find("workspace-inspector").bounds();
        let from = gpui::point(bounds.left() + px(2.), bounds.top() + px(100.));
        window.drag(from, from - gpui::point(px(50.), px(0.)), cx);
        window.render_frame(cx);
        let resized = inspector_width(window);
        assert!(resized > px(320.) && resized <= px(384.), "{resized:?}");
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = false;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(0.));
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), resized);
    })
    .unwrap();
}

#[gpui::test]
fn tab_switching_continues_the_current_panel_animation(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        gpui_router::use_navigate(window, cx).push("/ui");
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(320.));
        cx.set_reduce_motion(false);
        view.update(cx, |view, cx| view.new_tab(window, cx));
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(inspector_width(window), px(320.));
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(80));
    let middle = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let middle = inspector_width(window);
            assert!(middle > px(0.) && middle < px(320.));
            gpui_router::use_navigate(window, cx).push("/ui");
            window.render_frame(cx);
            assert_eq!(inspector_width(window), middle);
            middle
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(400));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(inspector_width(window) > middle);
        assert_eq!(inspector_width(window), px(320.));
    })
    .unwrap();
}

#[gpui::test]
fn closing_the_pending_neighbor_does_not_reopen_either_tab(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    let (first, second) = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            view.update(cx, |view, cx| {
                view.new_tab(window, cx);
                let first = view.pending_path.clone().unwrap();
                view.new_tab(window, cx);
                let second = view.pending_path.clone().unwrap();
                view.navigate(first.clone(), cx);
                (first, second)
            })
        })
        .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).tabs.active_id(), Some(first.as_str()));
        view.update(cx, |view, cx| {
            view.close_tab(&first, cx);
            view.close_tab(&second, cx);
        });
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let state = view.read(cx);
        assert!(state.tabs.path(&first).is_none());
        assert!(state.tabs.path(&second).is_none());
        let path = &gpui_router::use_location(cx).pathname;
        assert_ne!(path.as_ref(), first);
        assert_ne!(path.as_ref(), second);
    })
    .unwrap();
}

#[gpui::test]
fn resizing_a_moving_inspector_starts_at_its_displayed_width(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        gpui_router::use_navigate(window, cx).push("/projects");
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = true;
            cx.notify();
        });
        window.render_frame(cx);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(80));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let middle = inspector_width(window);
        assert!(middle > px(192.) && middle < px(320.), "{middle:?}");
        let bounds = window.find("workspace-inspector").bounds();
        let from = gpui::point(bounds.left() + px(2.), bounds.top() + px(100.));
        window.drag(from, from - gpui::point(px(20.), px(0.)), cx);
        window.render_frame(cx);
        cx.set_reduce_motion(true);
        window.render_frame(cx);
        let resized = inspector_width(window);
        assert!(
            resized > middle && resized <= middle + px(20.),
            "{middle:?} -> {resized:?}"
        );
        cx.set_reduce_motion(false);
        view.update(cx, |view, cx| {
            view.tabs
                .active_mut()
                .unwrap()
                .state
                .panels
                .inspector_visible = false;
            cx.notify();
        });
        window.render_frame(cx);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(10));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let middle = inspector_width(window);
        assert!(middle > px(192.) && middle < px(320.), "{middle:?}");
        let bounds = window.find("workspace-inspector").bounds();
        let from = gpui::point(bounds.left() + px(2.), bounds.top() + px(100.));
        window.drag(from, from - gpui::point(px(20.), px(0.)), cx);
        cx.set_reduce_motion(true);
        window.render_frame(cx);
        let resized = inspector_width(window);
        assert!(
            resized > middle && resized <= middle + px(20.),
            "{middle:?} -> {resized:?}"
        );
        assert!(
            view.read(cx)
                .tabs
                .active()
                .unwrap()
                .state
                .panels
                .inspector_visible
        );
    })
    .unwrap();
}

#[gpui::test]
fn tab_hover_keeps_the_glass_visible_and_keyboard_activation_routes(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        gpui_router::use_navigate(window, cx).push("/ui");
        window.render_frame(cx);
        let id = "select-tab-/ui";
        let bounds = window.find(id).bounds();
        window.hover(id, cx);
        window.render_frame(cx);
        assert_eq!(window.find(id).bounds(), bounds);
        let scaled = bounds.scale(window.scale_factor());
        let fill = window
            .painted_quads()
            .into_iter()
            .filter(|quad| quad.bounds == scaled)
            .filter_map(|quad| quad.background.as_solid())
            .last()
            .expect("tab hover paints a muted fill");
        assert_eq!(fill, cx.theme().colors.hover);
        window.click(id, cx);
        view.update(cx, |view, cx| view.navigate("/projects", cx));
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).tabs.active().unwrap().path, "/projects");
        window.press("enter", cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).tabs.active_id(), Some("/ui"));
        assert_eq!(gpui_router::use_location(cx).pathname.as_ref(), "/ui");
    })
    .unwrap();
}
