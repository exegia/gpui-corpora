use std::{cell::Cell, rc::Rc};

use gpui::{
    AppContext, Bounds, Context, Element, InputEvent, Render, TestAppContext, WindowBounds,
    WindowOptions, px, size,
};
use gpui_kit::test::TestWindowExt;

use super::*;

struct Harness {
    variant: ButtonVariant,
    size: ButtonSize,
    loading: bool,
    disabled: bool,
    clicks: Rc<Cell<usize>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let clicks = self.clicks.clone();
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap_4()
            .child(
                Button::new("target", "Open manuscript")
                    .variant(self.variant)
                    .size(self.size)
                    .loading(self.loading)
                    .disabled(self.disabled)
                    .icon(IconName::BookOpen)
                    .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
            )
            .child(Button::new("other", "Independent button"))
    }
}

fn setup(
    cx: &mut TestAppContext,
    width: f32,
    reduced: bool,
) -> (gpui::AnyWindowHandle, gpui::Entity<Harness>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        gpui_agentation::init_for_tests(cx);
        cx.set_reduce_motion(reduced);
    });
    cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(width), px(600.)),
                })),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|_| Harness {
                    variant: ButtonVariant::Default,
                    size: ButtonSize::Default,
                    loading: false,
                    disabled: false,
                    clicks: Rc::new(Cell::new(0)),
                })
            },
        )
        .expect("open button harness")
    })
}

#[gpui::test]
fn every_variant_activates_once_with_mouse_enter_and_space(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    for variant in ButtonVariant::ALL {
        cx.update_window(handle, |_, window, cx| {
            view.update(cx, |view, cx| {
                view.variant = variant;
                cx.notify();
            });
            window.render_frame(cx);
            let before = view.read(cx).clicks.get();
            window.click("target", cx);
            window.press("enter", cx);
            window.press("space", cx);
            assert_eq!(view.read(cx).clicks.get(), before + 3, "{variant:?}");
            assert_eq!(window.find("target").label(), Some("Open manuscript"));
        })
        .unwrap();
    }
}

#[cfg(target_os = "macos")]
#[gpui::test]
fn filled_and_outlined_variants_paint_glass(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    for variant in ButtonVariant::ALL {
        cx.update_window(handle, |_, window, cx| {
            view.update(cx, |view, cx| {
                view.variant = variant;
                cx.notify();
            });
            window.render_frame(cx);
        })
        .unwrap();
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let bounds = window.find("target").bounds().scale(window.scale_factor());
            let paints_glass = window
                .painted_backdrop_blurs()
                .iter()
                .any(|glass| glass.bounds == bounds);
            assert_eq!(
                paints_glass,
                !matches!(variant, ButtonVariant::Ghost | ButtonVariant::Link),
                "{variant:?}",
            );
        })
        .unwrap();
    }
}

#[cfg(target_os = "macos")]
#[gpui::test]
fn pressed_glass_scales_down_about_the_center_and_restores(cx: &mut TestAppContext) {
    fn glass_size(window: &Window) -> (f32, f32) {
        let bounds = window.find("target").bounds().scale(window.scale_factor());
        let blurs = window.painted_backdrop_blurs();
        let glass = blurs
            .iter()
            .find(|glass| {
                (glass.bounds.center().x.0 - bounds.center().x.0).abs() < 0.5
                    && (glass.bounds.center().y.0 - bounds.center().y.0).abs() < 0.5
                    && glass.bounds.size.width.0 <= bounds.size.width.0 + 0.5
            })
            .expect("button paints glass about its center");
        (glass.bounds.size.width.0, glass.bounds.size.height.0)
    }

    let (handle, _view) = setup(cx, 960., false);
    cx.executor().advance_clock(Duration::from_millis(400));
    let (resting, center) = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let bounds = window.find("target").bounds();
            let scaled = bounds.scale(window.scale_factor());
            assert_eq!(
                glass_size(window),
                (scaled.size.width.0, scaled.size.height.0)
            );
            window.hover("target", cx);
            window.dispatch_event(
                gpui::MouseDownEvent {
                    button: MouseButton::Left,
                    position: bounds.center(),
                    modifiers: Default::default(),
                    click_count: 1,
                    first_mouse: false,
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            (glass_size(window), bounds.center())
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(200));
    let pressed = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let pressed = glass_size(window);
            assert!(
                pressed.0 < resting.0 && pressed.1 < resting.1,
                "{pressed:?} vs {resting:?}"
            );
            window.dispatch_event(
                gpui::MouseUpEvent {
                    button: MouseButton::Left,
                    position: center,
                    modifiers: Default::default(),
                    click_count: 1,
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            pressed
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(200));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let released = glass_size(window);
        assert!(
            released.0 > pressed.0 && released.1 > pressed.1,
            "{released:?} vs {pressed:?}"
        );
    })
    .unwrap();
}

#[gpui::test]
fn disabled_and_loading_buttons_reject_activation_and_tab_focus(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    for variant in ButtonVariant::ALL {
        for loading in [false, true] {
            cx.update_window(handle, |_, window, cx| {
                view.update(cx, |view, cx| {
                    view.variant = variant;
                    view.disabled = !loading;
                    view.loading = loading;
                    cx.notify();
                });
                window.render_frame(cx);
                window.click("target", cx);
                window.click("other", cx);
                window.press("tab", cx);
                window.press("enter", cx);
                window.press("space", cx);
                assert_eq!(
                    view.read(cx).clicks.get(),
                    0,
                    "{variant:?}, loading={loading}"
                );
                assert_ne!(window.find("target").focused(), Some(true));
            })
            .unwrap();
        }
    }
}

#[gpui::test]
fn responsive_sizes_match_reference_and_icon_loading_preserves_square_bounds(
    cx: &mut TestAppContext,
) {
    for (width, heights) in [
        (960., [24., 28., 32., 36., 40., 24., 24., 32., 32., 36.]),
        (480., [28., 32., 36., 40., 44., 28., 28., 36., 32., 40.]),
    ] {
        let (handle, view) = setup(cx, width, true);
        for (button_size, height) in ButtonSize::ALL.into_iter().zip(heights) {
            cx.update_window(handle, |_, window, cx| {
                view.update(cx, |view, cx| {
                    view.size = button_size;
                    view.loading = false;
                    cx.notify();
                });
                window.render_frame(cx);
                let before = window.find("target").bounds();
                assert_eq!(before.size.height, px(height), "{button_size:?} at {width}");
                view.update(cx, |view, cx| {
                    view.loading = true;
                    cx.notify();
                });
                window.render_frame(cx);
                let after = window.find("target").bounds();
                assert_eq!(after.size.height, before.size.height);
                if button_size.is_icon() {
                    assert_eq!(before.size.width, px(height));
                    assert_eq!(before.size, after.size);
                } else {
                    assert!(after.size.width > before.size.width);
                }
            })
            .unwrap();
        }
    }
}

#[gpui::test]
fn text_links_use_inline_height_while_icon_links_keep_their_hit_area(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    cx.update_window(handle, |_, window, cx| {
        view.update(cx, |view, cx| {
            view.variant = ButtonVariant::Link;
            cx.notify();
        });
        window.render_frame(cx);
        assert!(window.find("target").bounds().size.height < px(32.));
        view.update(cx, |view, cx| {
            view.size = ButtonSize::Icon;
            cx.notify();
        });
        window.render_frame(cx);
        assert_eq!(window.find("target").bounds().size, size(px(32.), px(32.)));
    })
    .unwrap();
}

#[gpui::test]
fn loading_width_morphs_and_reverses_without_moving_the_label_out(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., false);
    let before = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let before = window.find("target").bounds();
            view.update(cx, |view, cx| {
                view.loading = true;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(window.find("target").bounds().size, before.size);
            before
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(150));
    let middle = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let middle = window.find("target").bounds();
            assert!(middle.size.width > before.size.width);
            assert_eq!(middle.size.height, before.size.height);
            let label = window.find((ElementId::from("target"), "content")).bounds();
            assert!(label.left() >= middle.left() && label.right() <= middle.right());
            view.update(cx, |view, cx| {
                view.loading = false;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(window.find("target").bounds().size, middle.size);
            middle
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(300));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("target").bounds().size, before.size);
        assert!(middle.size.width < before.size.width + px(24.));
        window.click("target", cx);
        assert_eq!(view.read(cx).clicks.get(), 1);
    })
    .unwrap();
}

#[gpui::test]
fn accessibility_exposes_disabled_busy_and_link_semantics(cx: &mut TestAppContext) {
    setup(cx, 960., true);
    struct Probe;
    impl Render for Probe {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::canvas(
                |_, window, cx| {
                    for (disabled, loading) in [(false, false), (true, false), (false, true)] {
                        let element = Button::new("semantics", "Save manuscript")
                            .disabled(disabled)
                            .loading(loading)
                            .render(window, cx)
                            .into_element();
                        let mut node = gpui::accesskit::Node::new(element.a11y_role().unwrap());
                        element.write_a11y_info(&mut node);
                        assert_eq!(node.label(), Some("Save manuscript"));
                        assert_eq!(node.is_disabled(), disabled || loading);
                        assert_eq!(node.is_busy(), loading);
                    }
                    let link = Button::new("link-semantics", "Documentation")
                        .href("https://example.com")
                        .render(window, cx)
                        .into_element();
                    assert_eq!(link.a11y_role(), Some(Role::Link));
                },
                |_, _, _, _| {},
            )
            .size_full()
        }
    }
    let (_, visual) = cx.add_window_view(|_, _| Probe);
    visual.update(|window, cx| {
        window.draw(cx).clear(cx);
    });
}

#[gpui::test]
fn resizing_and_press_feedback_preserve_the_surrounding_layout(cx: &mut TestAppContext) {
    fn background(window: &Window) -> gpui::Hsla {
        let bounds = window.find("target").bounds().scale(window.scale_factor());
        #[cfg(target_os = "macos")]
        return window
            .painted_backdrop_blurs()
            .iter()
            .find(|glass| {
                // Pressed glass shrinks about the button's center.
                let (gx, gy) = (glass.bounds.origin.x.0, glass.bounds.origin.y.0);
                let (gw, gh) = (glass.bounds.size.width.0, glass.bounds.size.height.0);
                let (x, y) = (bounds.origin.x.0, bounds.origin.y.0);
                let (w, h) = (bounds.size.width.0, bounds.size.height.0);
                gx >= x - 0.5
                    && gy >= y - 0.5
                    && gx + gw <= x + w + 0.5
                    && gy + gh <= y + h + 0.5
                    && gw >= w * 0.9
                    && gh >= h * 0.9
            })
            .expect("button paints glass inside its bounds")
            .tint;
        #[cfg(not(target_os = "macos"))]
        window
            .painted_quads()
            .iter()
            .find(|quad| quad.bounds == bounds)
            .expect("button paints its fallback inside its bounds")
            .background
            .as_solid()
            .expect("button fallback uses a solid semantic tint")
    }

    let (handle, view) = setup(cx, 960., false);
    let initial = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let initial = window.find("target").bounds();
            view.update(cx, |view, cx| {
                view.size = ButtonSize::Xl;
                cx.notify();
            });
            window.render_frame(cx);
            assert_eq!(
                window.find("target").bounds().size.height,
                initial.size.height
            );
            initial
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(150));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let middle = window.find("target").bounds().size.height;
        assert!(middle > initial.size.height && middle < px(40.));
    })
    .unwrap();
    cx.executor().advance_clock(Duration::from_millis(150));
    let (resting, sibling) = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            let resting = window.find("target").bounds();
            assert_eq!(resting.size.height, px(40.));
            let sibling = window.find("other").bounds();
            window.hover("target", cx);
            window.dispatch_event(
                gpui::MouseDownEvent {
                    button: MouseButton::Left,
                    position: resting.center(),
                    modifiers: Default::default(),
                    click_count: 1,
                    first_mouse: false,
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            (resting, sibling)
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(150));
    let pressed_background = cx
        .update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            assert_eq!(window.find("target").bounds().size, resting.size);
            assert_eq!(window.find("other").bounds(), sibling);
            let pressed_background = background(window);
            assert!(pressed_background.a > 0., "pressed {pressed_background:?}",);
            window.dispatch_event(
                gpui::MouseUpEvent {
                    button: MouseButton::Left,
                    position: resting.center(),
                    modifiers: Default::default(),
                    click_count: 1,
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            pressed_background
        })
        .unwrap();
    cx.executor().advance_clock(Duration::from_millis(150));
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(background(window), pressed_background);
        assert_eq!(window.find("target").bounds().size, resting.size);
        assert_eq!(window.find("other").bounds(), sibling);
    })
    .unwrap();
}

#[gpui::test]
fn composed_icons_and_labels_stay_inside_the_button_box(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    for variant in [
        ButtonVariant::Outline,
        ButtonVariant::Default,
        ButtonVariant::Link,
    ] {
        for size in [ButtonSize::Xs, ButtonSize::Default, ButtonSize::Xl] {
            cx.update_window(handle, |_, window, cx| {
                view.update(cx, |view, cx| {
                    view.variant = variant;
                    view.size = size;
                    cx.notify();
                });
                window.render_frame(cx);
                let root = window.find("target").bounds();
                let content = window.find((ElementId::from("target"), "content")).bounds();
                assert!(
                    content.left() >= root.left() && content.right() <= root.right(),
                    "{variant:?} {size:?}: {root:?} {content:?}"
                );
            })
            .unwrap();
        }
    }
}

#[gpui::test]
fn light_button_colors_match_figma_and_reach_the_glass(cx: &mut TestAppContext) {
    let (handle, _) = setup(cx, 960., true);
    cx.update_window(handle, |_, _, cx| {
        let palette = crate::styles::tokens::DefaultTheme::new().light;
        crate::styles::tokens::DefaultTheme::new().install(cx);
        crate::ely::theme::Theme::set_mode_now(crate::ely::theme::Mode::Light, cx);
        let theme = cx.theme().clone();
        let rgb = |hex| gpui::Hsla::from(gpui::rgb(hex));
        for (variant, tint, alpha, foreground) in [
            // Default follows the theme's accent roles, not fixed hex values.
            (ButtonVariant::Default, palette.accent, 0.95, palette.on_accent),
            (ButtonVariant::Secondary, rgb(0x1a1a1a), 0.87, rgb(0xf1f1f1)),
            (ButtonVariant::Destructive, rgb(0xe7000b), 0.83, rgb(0xf1f1f1)),
            (ButtonVariant::DestructiveOutline, rgb(0xae0000), 0.01, rgb(0x7a0006)),
        ] {
            let resting = variant.colors(&theme, false, false);
            assert_eq!(resting.glass_tint, tint.opacity(alpha));
            assert_eq!(resting.foreground, foreground);
            let spec = variant
                .glass_spec(&theme, resting.glass_tint, resting.engage)
                .unwrap();
            assert_eq!(spec.tint, Some(resting.glass_tint));
            assert_eq!(spec.gain_light, 1. - alpha);
            let hovered = variant.colors(&theme, true, false);
            assert!(hovered == variant.colors(&theme, false, true));
            assert!(hovered.glass_tint != resting.glass_tint);
        }
        let outline = ButtonVariant::Outline.colors(&theme, false, false);
        assert_eq!(outline.border.a, 0.16);
        assert_eq!(
            ButtonVariant::Outline.colors(&theme, true, false).border.a,
            0.41
        );
        assert_eq!(
            ButtonVariant::Ghost
                .colors(&theme, true, false)
                .background
                .a,
            0.42
        );
    })
    .unwrap();
}

#[gpui::test]
fn icon_only_buttons_center_content_at_every_size(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx, 960., true);
    for size in ButtonSize::ALL.into_iter().filter(|size| size.is_icon()) {
        for loading in [false, true] {
            cx.update_window(handle, |_, window, cx| {
                view.update(cx, |view, cx| {
                    view.size = size;
                    view.loading = loading;
                    cx.notify();
                });
                window.render_frame(cx);
                let root = window.find("target").bounds();
                let content = window.find((ElementId::from("target"), "content")).bounds();
                assert_eq!(root.size.width, root.size.height, "{size:?}");
                assert!(
                    (content.center().x - root.center().x).abs() < px(0.1),
                    "{size:?}"
                );
                assert!(
                    (content.center().y - root.center().y).abs() < px(0.1),
                    "{size:?}"
                );
                #[cfg(target_os = "macos")]
                {
                    let bounds = root.scale(window.scale_factor());
                    let blurs = window.painted_backdrop_blurs();
                    let glass = blurs.iter().find(|glass| glass.bounds == bounds).unwrap();
                    assert_eq!(glass.corner_radii.top_left, bounds.size.height / 2.);
                }
            })
            .unwrap();
        }
    }
}
