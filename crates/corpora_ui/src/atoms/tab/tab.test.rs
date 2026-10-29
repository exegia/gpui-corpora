use gpui::{AppContext, Context, Render, TestAppContext};
use gpui_kit::test::TestWindowExt;

use super::*;
use crate::ely::theme::ActiveTheme as _;

struct Harness {
    selected: usize,
    closed: Option<usize>,
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
            .child(
                Tabs::new("horizontal-tabs")
                    .variant(TabVariant::Horizontal)
                    .items([
                        Tab::new("UI").icon(div().size_2()),
                        Tab::new("New tab 1").icon(div().size_2()).closable(true),
                        Tab::new("New tab 2").icon(div().size_2()).closable(true),
                    ])
                    .selected_index(self.selected)
                    .on_change(cx.listener(|this, index, _, cx| {
                        this.selected = *index;
                        cx.notify();
                    }))
                    .on_close(cx.listener(|this, index, _, cx| {
                        this.closed = Some(*index);
                        cx.notify();
                    })),
            )
            .child(Tabs::new("other-tabs").items([Tab::new("Independent")]))
            .child(
                Tabs::new("unselected-tabs")
                    .selected_index(None::<usize>)
                    .items([Tab::new("Available")]),
            )
            .child(Tabs::new("empty-tabs"))
    }
}

fn setup(cx: &mut TestAppContext, reduced: bool) -> (gpui::AnyWindowHandle, gpui::Entity<Harness>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        gpui_agentation::init_for_tests(cx);
        cx.set_reduce_motion(reduced);
        gpui_kit::open_window(Default::default(), cx, |_, cx| {
            cx.new(|_| Harness {
                selected: 0,
                closed: None,
            })
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
        window.within("tabs").hover(1usize, cx);
        window.render_frame(cx);
        let hover_fill = window
            .painted_quads()
            .into_iter()
            .filter(|quad| quad.bounds == hover_bounds.scale(window.scale_factor()))
            .filter_map(|quad| quad.background.as_solid())
            .last()
            .expect("tab hover paints a muted fill");
        assert_eq!(hover_fill, cx.theme().colors.hover);
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
            window.within("unselected-tabs").find(0usize).selected(),
            Some(false)
        );
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
fn horizontal_tabs_render_glass_and_close_the_requested_item(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.within("horizontal-tabs").click(1usize, cx);
        window.render_frame(cx);

        let tab = window.within("horizontal-tabs").find(1usize);
        assert_eq!(tab.label(), Some("New tab 1"));
        assert_eq!(tab.selected(), Some(true));
        #[cfg(target_os = "macos")]
        let bounds = tab.bounds().scale(window.scale_factor());
        #[cfg(target_os = "macos")]
        assert!(
            window
                .painted_backdrop_blurs()
                .iter()
                .any(|glass| glass.bounds == bounds && glass.blur_radius.0 > 0.)
        );

        let close_id = (ElementId::from("horizontal-tabs"), "close-1");
        let close = window.find(close_id.clone());
        assert_eq!(close.role(), Some(Role::Button));
        assert_eq!(close.label(), Some("Close New tab 1"));
        window.click(close_id, cx);
        assert_eq!(view.read(cx).closed, Some(1));
    })
    .unwrap();
}

#[cfg(target_os = "macos")]
#[gpui::test]
fn horizontal_glass_follows_selection_and_hover(cx: &mut TestAppContext) {
    fn glass_count(window: &mut Window, tab: &'static str, index: usize) -> usize {
        let bounds = window
            .within(tab)
            .find(index)
            .bounds()
            .scale(window.scale_factor());
        window
            .painted_backdrop_blurs()
            .iter()
            .filter(|glass| glass.bounds == bounds)
            .count()
    }

    let (handle, _view) = setup(cx, true);
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        // Tab 0 is selected; tab 2 is neither selected nor hovered.
        assert_eq!(glass_count(window, "horizontal-tabs", 0), 1);
        assert_eq!(glass_count(window, "horizontal-tabs", 2), 0);

        window.within("horizontal-tabs").hover(2usize, cx);
        window.render_frame(cx);
        assert_eq!(glass_count(window, "horizontal-tabs", 2), 1);
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
