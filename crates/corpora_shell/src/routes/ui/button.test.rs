use gpui::{AppContext, Bounds, TestAppContext, WindowBounds, WindowOptions, px, size};
use gpui_kit::test::TestWindowExt;

use super::*;

fn id(group: &'static str, name: &'static str) -> (gpui::ElementId, &'static str) {
    (group.into(), name)
}

#[gpui::test]
fn playground_controls_cover_variants_sizes_and_inert_states(cx: &mut TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        cx.set_reduce_motion(true);
    });
    let (handle, view) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(2200.)),
                })),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(|_| ButtonShowcase::new()),
        )
        .unwrap()
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        for variant in ButtonVariant::ALL {
            window.click(id("variant", variant.name()), cx);
            assert_eq!(view.read(cx).variant.name(), variant.name());
        }
        for size in ButtonSize::ALL {
            window.click(id("size", size.name()), cx);
            assert_eq!(view.read(cx).size, size);
        }
        assert_eq!(
            window.find("button-preview").label(),
            Some("Open manuscript")
        );
        window.click("button-preview", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).clicks, 2);
        window.click(id("loading", "true"), cx);
        window.click("button-preview", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).clicks, 2);
        window.click(id("loading", "false"), cx);
        window.click(id("disabled", "true"), cx);
        window.click("button-preview", cx);
        assert_eq!(view.read(cx).clicks, 2);
        window.click(id("disabled", "false"), cx);
        window.click("button-preview", cx);
        assert_eq!(view.read(cx).clicks, 3);
    })
    .unwrap();
}
