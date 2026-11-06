use gpui::{AppContext, Bounds, TestAppContext, WindowBounds, WindowOptions, px, size};
use gpui_kit::test::TestWindowExt;

use super::*;

fn setup(cx: &mut TestAppContext) -> (gpui::AnyWindowHandle, Entity<Ui>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        cx.set_reduce_motion(true);
    });
    cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(960.0), px(1400.0)),
                })),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| Ui::new(window, cx)),
        )
        .expect("open UI showcase")
    })
}

fn run(cx: &mut TestAppContext, test: impl FnOnce(&Entity<Ui>, &mut Window, &mut App)) {
    let (window, view) = setup(cx);
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
        test(&view, window, cx);
    })
    .expect("update UI showcase");
}

#[gpui::test]
fn toc_search_selects_matching_location_and_returns_focus(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click("contents", cx);
        window.input("preconfig", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selected.as_ref(), "Preconfigured components");
    })
    .expect("choose location");
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("contents").focused(), Some(true));
        window.press("enter", cx);
        assert_eq!(
            view.read(cx).contents.read(cx).filtered(cx),
            vec!["Preconfigured components"]
        );
    })
    .expect("check restored focus");
}

#[gpui::test]
fn enter_on_a_focused_menu_row_chooses_that_row(cx: &mut TestAppContext) {
    run(cx, |view, window, cx| {
        window.click("sections", cx);
        window.press("tab", cx);
        window.press("tab", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selected.as_ref(), "Examples");
    });
}

#[gpui::test]
fn menu_search_can_be_empty_and_arrow_selection_wraps(cx: &mut TestAppContext) {
    run(cx, |view, window, cx| {
        window.click("sections", cx);
        window.input("no matching location", cx);
        assert!(view.read(cx).sections.read(cx).filtered(cx).is_empty());
        window.press("enter", cx);
        assert_eq!(view.read(cx).selected.as_ref(), "No location selected");
        window.press("secondary-a", cx);
        window.press("backspace", cx);
        window.press("up", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selected.as_ref(), "API notes");
    });
}

#[gpui::test]
fn editing_current_page_updates_the_trail(cx: &mut TestAppContext) {
    run(cx, |view, window, cx| {
        view.read(cx).current.focus_handle(cx).focus(window, cx);
        window.press("secondary-a", cx);
        window.input("Chapter 12", cx);
        assert_eq!(window.find("current").label(), Some("Chapter 12"));
        assert_eq!(window.find("library").label(), Some("Chapter 12"));
        assert_eq!(window.find("composition-page").label(), Some("Chapter 12"));
    });
}

#[gpui::test]
fn bible_location_keeps_current_items_noninteractive(cx: &mut TestAppContext) {
    run(cx, |view, window, cx| {
        for id in ["bible-book", "bible-chapter", "bible-verse"] {
            window.click(id, cx);
            assert_ne!(window.find(id).focused(), Some(true));
        }
        assert_eq!(view.read(cx).selected.as_ref(), "No location selected");
    });
}
