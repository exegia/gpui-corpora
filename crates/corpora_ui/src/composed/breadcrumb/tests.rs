use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{AppContext, Bounds, Context, Render, TestAppContext, WindowOptions, px, size};
use gpui_kit::test::{TestSupportExt, TestWindowExt};

use super::*;

struct Example {
    clicks: Rc<Cell<usize>>,
    menu_clicks: Rc<Cell<usize>>,
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let clicks = self.clicks.clone();
        let default_clicks = self.clicks.clone();
        let menu_clicks = self.menu_clicks.clone();
        let toc_clicks = self.clicks.clone();
        let popup_clicks = self.menu_clicks.clone();
        Breadcrumb::new("trail").label("Test location").child(
            BreadcrumbList::new()
                .child(
                    BreadcrumbItem::link("home", "Home", "/home")
                        .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
                )
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::toc(
                    "contents",
                    "Contents",
                    move |_, _, _| {
                        let toc_clicks = toc_clicks.clone();
                        div()
                            .id("toc-body")
                            .test_support()
                            .w_32()
                            .h_16()
                            .child("Chapters")
                            .child(
                                BreadcrumbTrigger::new("toc-choice", "Chapter one").on_click(
                                    move |_, _, _| {
                                        toc_clicks.set(toc_clicks.get() + 1);
                                    },
                                ),
                            )
                    },
                ))
                .child(BreadcrumbSeparator::menu(
                    "separator",
                    "Choose location",
                    move |trigger| {
                        BreadcrumbPopup::new("separator-popup", trigger, |_, _, _| {
                            div()
                                .id("menu-body")
                                .test_support()
                                .w_32()
                                .h_16()
                                .child("Locations")
                        })
                    },
                ))
                .child(BreadcrumbItem::menu("menu", "Sections", move |trigger| {
                    trigger.on_click(move |_, _, _| menu_clicks.set(menu_clicks.get() + 1))
                }))
                .child(BreadcrumbSeparator::new().symbol(BreadcrumbSeparatorSymbol::Slash))
                .child(
                    BreadcrumbSeparator::menu(
                        "disabled-menu",
                        "Unavailable locations",
                        |trigger| {
                            BreadcrumbPopup::new("disabled-popup", trigger, |_, _, _| {
                                div()
                                    .id("disabled-body")
                                    .test_support()
                                    .child("Must stay closed")
                            })
                        },
                    )
                    .disabled(true),
                )
                .child(
                    BreadcrumbItem::new("current", "Library", BreadcrumbPage::new("Library"))
                        .on_click(move |_, _, _| default_clicks.set(default_clicks.get() + 100)),
                )
                .child(
                    BreadcrumbItem::menu("callback-menu", "More sections", |trigger| {
                        BreadcrumbPopup::new("callback-popup", trigger, |_, _, _| {
                            div()
                                .id("long-body")
                                .test_support()
                                .flex()
                                .flex_col()
                                .w_32()
                                .child(BreadcrumbTrigger::new("first-choice", "First"))
                                .child(div().h(px(1200.0)).flex_none())
                                .child(BreadcrumbTrigger::new("last-choice", "Last"))
                        })
                    })
                    .on_click(move |_, _, _| popup_clicks.set(popup_clicks.get() + 1)),
                ),
        )
    }
}

fn run(
    cx: &mut TestAppContext,
    test: impl FnOnce(&mut Window, &mut App, Rc<Cell<usize>>, Rc<Cell<usize>>),
) {
    let (window, clicks, menu_clicks) = setup(cx, true);
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        test(window, cx, clicks, menu_clicks);
    })
    .expect("update test window");
}

fn setup(
    cx: &mut TestAppContext,
    reduce_motion: bool,
) -> (gpui::AnyWindowHandle, Rc<Cell<usize>>, Rc<Cell<usize>>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        gpui_agentation::ely::init_for_tests(cx);
        cx.set_reduce_motion(reduce_motion);
    });
    let clicks = Rc::new(Cell::new(0));
    let menu_clicks = Rc::new(Cell::new(0));
    let (window, _) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(640.), px(480.)),
                })),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|_| Example {
                    clicks: clicks.clone(),
                    menu_clicks: menu_clicks.clone(),
                })
            },
        )
        .expect("open test window")
    });
    cx.update_window(window, |_, window, _| window.activate_window())
        .unwrap();
    cx.run_until_parked();
    (window, clicks, menu_clicks)
}

#[gpui::test]
fn popup_slides_down_on_open_and_up_before_unmounting(cx: &mut TestAppContext) {
    let (handle, _, _) = setup(cx, false);
    let start = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.click("contents", cx);
            window.find("toc-body").bounds()
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(120));
    let middle = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            window.find("toc-body").bounds()
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(120));
    let end = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let end = window.find("toc-body").bounds();
            window.press("escape", cx);
            assert!(window.try_find("toc-body").is_some());
            assert_eq!(window.find("toc-body").bounds(), end);
            end
        })
        .unwrap();
    assert!(start.top() < middle.top() && middle.top() < end.top());
    assert_eq!(start.size, end.size);
    cx.executor().advance_clock(Duration::from_millis(120));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let closing = window.find("toc-body").bounds();
        assert!(closing.top() < end.top() && closing.top() > start.top());
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(120));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(window.try_find("toc-body").is_none());
        assert_eq!(window.find("contents").focused(), Some(true));
    })
    .unwrap();
}

#[gpui::test]
fn reopening_during_exit_continues_from_the_current_position(cx: &mut TestAppContext) {
    let (handle, _, _) = setup(cx, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("contents", cx);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(240));
    let settled = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let settled = window.find("toc-body").bounds();
            window.press("escape", cx);
            settled
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(60));
    let reversed = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let closing = window.find("toc-body").bounds();
            window.click("contents", cx);
            assert_eq!(window.find("toc-body").bounds(), closing);
            closing
        })
        .unwrap();
    assert!(reversed.top() < settled.top());
    cx.executor().advance_clock(Duration::from_millis(240));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("toc-body").bounds(), settled);
    })
    .unwrap();
}

#[gpui::test]
fn exiting_popup_rejects_clicks_and_tab_focus(cx: &mut TestAppContext) {
    let (handle, clicks, _) = setup(cx, false);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("contents", cx);
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(240));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("toc-choice", cx);
        assert_eq!(clicks.get(), 1);
        window.press("escape", cx);
    })
    .unwrap();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("toc-choice", cx);
        assert_eq!(clicks.get(), 1);
    })
    .unwrap();
    for _ in 0..6 {
        cx.update_window(handle, |_, window, cx| window.press("tab", cx))
            .unwrap();
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            assert_ne!(window.find("toc-choice").focused(), Some(true));
        })
        .unwrap();
    }
}

#[gpui::test]
fn link_and_menu_activate_but_current_page_does_not(cx: &mut TestAppContext) {
    run(cx, |window, cx, clicks, menu_clicks| {
        window.click("home", cx);
        window.click("current", cx);
        window.click("menu", cx);
        assert_eq!((clicks.get(), menu_clicks.get()), (1, 1));
    });
}

#[gpui::test]
fn popup_menu_preserves_the_trigger_callback(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, menu_clicks| {
        assert_eq!(window.find("callback-menu").expanded(), Some(false));
        window.click("callback-menu", cx);
        assert_eq!(menu_clicks.get(), 1);
        assert_eq!(window.find("callback-menu").expanded(), Some(true));
        assert!(window.find("first-choice").visible());
        window.press("escape", cx);
        window.press("enter", cx);
        assert_eq!(menu_clicks.get(), 2);
        assert_eq!(window.find("callback-menu").expanded(), Some(true));
    });
}

#[gpui::test]
fn long_popup_content_scrolls_inside_the_viewport(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, _| {
        window.click("callback-menu", cx);
        let first = window.find("first-choice").bounds();
        assert!(first.top() >= px(0.0));
        assert!(first.bottom() < window.viewport_size().height);
        window.scroll(
            "first-choice",
            gpui::ScrollDelta::Pixels(gpui::point(px(0.0), px(-1400.0))),
            cx,
        );
        let last = window.find("last-choice").bounds();
        assert!(last.top() >= px(0.0));
        assert!(last.bottom() <= window.viewport_size().height);
    });
}

#[gpui::test]
fn toc_opens_below_trigger_and_escape_restores_focus(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, _| {
        window.click("home", cx);
        window.press("tab", cx);
        assert_eq!(window.find("contents").focused(), Some(true));
        window.press("enter", cx);
        assert!(
            window.find("toc-body").bounds().top() >= window.find("contents").bounds().bottom()
        );
        window.press("escape", cx);
        assert!(window.try_find("toc-body").is_none());
        window.press("enter", cx);
        assert!(window.find("toc-body").visible());
    });
}

#[gpui::test]
fn separator_menu_opens_and_outside_click_dismisses(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, _| {
        window.click("separator", cx);
        assert!(window.find("menu-body").visible());
        window.click("home", cx);
        assert!(window.try_find("menu-body").is_none());
    });
}

#[gpui::test]
fn disabled_menu_stays_closed_and_icon_trigger_keeps_its_name(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, _| {
        assert_eq!(window.find("separator").label(), Some("Choose location"));
        assert_eq!(window.find("current").label(), Some("Library"));
        window.click("disabled-menu", cx);
        assert!(window.try_find("disabled-body").is_none());
    });
}

#[gpui::test]
fn zoomed_dark_trail_wraps_and_keeps_popover_inside_viewport(cx: &mut TestAppContext) {
    run(cx, |window, cx, _, _| {
        crate::ely::theme::Theme::set_mode_now(crate::ely::theme::Mode::Dark, cx);
        gpui_kit::component::Theme::global_mut(cx).font_size = px(28.0);
        window.render_frame(cx);
        assert!(window.find("current").bounds().top() > window.find("home").bounds().top());
        assert!(window.find("current").bounds().right() <= window.viewport_size().width);
        window.click("contents", cx);
        let popup = window.find("toc-body").bounds();
        assert!(popup.left() >= px(0.0) && popup.right() <= window.viewport_size().width);
    });
}
