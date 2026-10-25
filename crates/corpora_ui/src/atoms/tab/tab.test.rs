use gpui::{AppContext, Context, Render, TestAppContext};
use gpui_kit::test::TestWindowExt;

use super::*;

struct Harness {
    selected: usize,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(
                Tabs::new("tabs")
                    .items([
                        Tab::new("General"),
                        Tab::new("Workspace"),
                        Tab::new("Developer"),
                    ])
                    .selected_index(self.selected)
                    .on_change(cx.listener(|this, index, _, cx| {
                        this.selected = *index;
                        cx.notify();
                    })),
            )
            .child(Tabs::new("other-tabs").items([Tab::new("Independent")]))
            .child(Tabs::new("empty-tabs"))
    }
}

fn setup(cx: &mut TestAppContext, reduced: bool) -> (gpui::AnyWindowHandle, gpui::Entity<Harness>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        gpui_agentation::init_for_tests(cx);
        cx.set_reduce_motion(reduced);
        gpui_kit::open_window(Default::default(), cx, |_, cx| {
            cx.new(|_| Harness { selected: 0 })
        })
        .expect("open tabs harness")
    })
}

fn indicator(window: &mut Window) -> gpui::Bounds<gpui::Pixels> {
    let bounds = window
        .find((ElementId::from("tabs"), "selected-indicator"))
        .bounds();
    #[cfg(target_os = "macos")]
    assert!(window.painted_backdrop_blurs().iter().any(|glass| {
        glass.bounds == bounds.scale(window.scale_factor())
            && glass.lens.0 > 0.
            && glass.magnify > 1.3
            && glass.dispersion >= 0.02
            && glass.edge > 0.3
            && glass.edge_width.0 >= 2.
    }));
    bounds
}

#[gpui::test]
fn pointer_and_keyboard_select_tabs_with_roving_focus(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let hover_bounds = window.within("tabs").find(1usize).bounds();
        window.hover(1usize, cx);
        window.render_frame(cx);
        let hover_fill = window
            .painted_quads()
            .into_iter()
            .filter(|quad| quad.bounds == hover_bounds.scale(window.scale_factor()))
            .filter_map(|quad| quad.background.as_solid())
            .last()
            .expect("tab hover paints a muted fill");
        assert_eq!(hover_fill, cx.theme().colors.fg_muted.opacity(0.04));
        window.within("tabs").click(1usize, cx);
        assert_eq!(view.read(cx).selected, 1);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 2);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 0);
        window.press("left", cx);
        assert_eq!(view.read(cx).selected, 2);
        window.press("home", cx);
        assert_eq!(view.read(cx).selected, 0);
        window.press("end", cx);
        window.press("space", cx);
        window.press("enter", cx);
        assert_eq!(view.read(cx).selected, 2);
        window.render_frame(cx);
        assert_eq!(window.within("tabs").find(2usize).role(), Some(Role::Tab));
        assert_eq!(
            window.within("tabs").find(2usize).label(),
            Some("Developer")
        );
        assert_eq!(window.within("tabs").find(2usize).selected(), Some(true));
        assert_eq!(window.within("tabs").find(1usize).selected(), Some(false));
        assert_eq!(
            indicator(window),
            window.within("tabs").find(2usize).bounds()
        );
        let other = window
            .find((ElementId::from("other-tabs"), "selected-indicator"))
            .bounds();
        assert_eq!(other, window.within("other-tabs").find(0usize).bounds());
    })
    .unwrap();
}

#[gpui::test]
fn selection_spring_moves_and_retargets_without_jumping(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, false);
    let initial = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let initial = indicator(window);
            window.within("tabs").click(2usize, cx);
            window.render_frame(cx);
            assert_eq!(indicator(window), initial);
            initial
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(80));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let middle = indicator(window);
        let target = window.within("tabs").find(2usize).bounds();
        assert!(middle.left() > initial.left() && middle.left() < target.left());
        view.update(cx, |view, cx| {
            view.selected = 0;
            cx.notify();
        });
        window.render_frame(cx);
        assert!((indicator(window).left() - middle.left()).abs() < px(1.));
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_secs(2));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(indicator(window), initial);
    })
    .unwrap();
}

#[gpui::test]
fn reduced_motion_snaps_and_out_of_range_selection_stays_in_bounds(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        view.update(cx, |view, cx| {
            view.selected = usize::MAX;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(
            indicator(window),
            window.within("tabs").find(2usize).bounds()
        );
    })
    .unwrap();
}
