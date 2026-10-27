use gpui::{
    AppContext, Bounds, ElementId, TestAppContext, WindowBounds, WindowOptions, point, px, size,
};
use gpui_kit::test::TestWindowExt;

use super::*;

fn id(group: &'static str, name: &'static str) -> (ElementId, &'static str) {
    (group.into(), name)
}

#[gpui::test]
fn material_probe_controls_update_and_reset_the_glass(cx: &mut TestAppContext) {
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
                    size: size(px(1100.), px(1800.)),
                })),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(GlassShowcase::new),
        )
        .expect("open glass material probe")
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        #[cfg(target_os = "macos")]
        assert!(!window.painted_backdrop_blurs().is_empty());

        window.click(id("glass-backdrop", "coded"), cx);
        assert_eq!(view.read(cx).backdrop, Backdrop::Coded);

        let width_slider = view.read(cx).sliders.get(ProbeControl::Width).clone();
        width_slider.update(cx, |slider, cx| {
            slider.set_value(240., window, cx);
            cx.emit(SliderEvent::Change(slider.value()));
        });
    })
    .expect("move width slider");
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(view.read(cx).width, 240.);
        assert_eq!(view.read(cx).preset, Preset::Custom);

        window.click("glass-tint-switch", cx);
        assert!(view.read(cx).tint_enabled);
        window.press("space", cx);
        assert!(!view.read(cx).tint_enabled);

        window.click(id("glass-tint-color", "danger"), cx);
        window.click(id("glass-background-color", "sunken"), cx);
        assert_eq!(view.read(cx).tint_color, ThemeColor::Danger);
        assert_eq!(view.read(cx).background_color, ThemeColor::Sunken);

        let probe = window.find("glass-probe").bounds();
        let before = view.read(cx).position;
        window.drag(probe.center(), probe.center() + point(px(40.), px(24.)), cx);
        window.render_frame(cx);
        assert_ne!(view.read(cx).position, before);

        window.click(id("glass-preset", "deep"), cx);
        assert_eq!(view.read(cx).preset, Preset::Deep);
        assert_eq!(view.read(cx).spec.magnify, 1.38);
        assert_eq!(
            view.read(cx)
                .sliders
                .get(ProbeControl::Magnify)
                .read(cx)
                .value()
                .start(),
            1.38
        );

        window.click("glass-reset", cx);
        let state = view.read(cx);
        assert_eq!(state.backdrop, Backdrop::Text);
        assert_eq!(state.preset, Preset::Glass);
        assert_eq!(state.background_color, ThemeColor::Background);
        assert_eq!(state.tint_color, ThemeColor::Accent);
        assert_eq!(state.position, point(px(120.), px(80.)));
        assert_eq!(state.width, 168.);
        assert!(!state.tint_enabled);
        assert_eq!(state.spec.magnify, GlassSpec::default().magnify);
    })
    .expect("exercise glass material probe");
    cx.run_until_parked();
    cx.update(|cx| {
        let state = view.read(cx);
        assert_eq!(state.preset, Preset::Glass);
        assert_eq!(state.width, 168.);
    });
}
