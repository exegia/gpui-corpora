use crate::ely::theme::{ActiveTheme, ControlSize, TextSize};
use crate::styles::palette::color;
use gpui::{App, BoxShadow, FontWeight, IntoElement, RenderOnce, Window, div, prelude::*, px};

/// The shared unread count pill, capped at 99+.
#[derive(IntoElement)]
pub struct UnreadBadge {
    count: usize,
}

impl UnreadBadge {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

fn label(count: usize) -> String {
    if count > 99 {
        "99+".into()
    } else {
        count.to_string()
    }
}

impl RenderOnce for UnreadBadge {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let height = theme.control_height(ControlSize::Sm) * 0.67;
        let radius = height.to_pixels(window.rem_size()) / 2.;
        div()
            .relative()
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .min_w(height)
            .h(height)
            .px_1()
            .rounded_full()
            .text_size(theme.text_size(TextSize::Xs) * (10. / 11.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(color("on-danger", theme.mode()))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(crate::ui::glass::tinted(radius, colors.danger.opacity(0.7))),
            )
            .child(div().absolute().inset_0().rounded_full().shadow(vec![
                BoxShadow::new(px(0.), px(1.), colors.on_media.opacity(0.55))
                    .blur_radius(px(1.)).inset(),
                BoxShadow::new(px(0.), px(-1.), colors.shadow.opacity(0.35))
                    .blur_radius(px(1.)).inset(),
            ]))
            .child(div().relative().child(label(self.count)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[gpui::test]
    fn badge_instances_share_theme_tint_and_material(cx: &mut gpui::TestAppContext) {
        use gpui::{AppContext, Context, Render};
        use gpui_kit::test::TestWindowExt;
        struct Pair;
        impl Render for Pair {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .flex()
                    .gap_2()
                    .child(UnreadBadge::new(3))
                    .child(UnreadBadge::new(3))
            }
        }
        let (handle, _) = cx.update(|cx| {
            gpui_kit::init(cx);
            crate::ely::init_for_tests(cx);
            crate::styles::tokens::DefaultTheme::new().install(cx);
            gpui_kit::open_window(Default::default(), cx, |_, cx| cx.new(|_| Pair)).unwrap()
        });
        for mode in [
            crate::ely::theme::Mode::Light,
            crate::ely::theme::Mode::Dark,
        ] {
            cx.update(|cx| crate::ely::theme::Theme::set_mode_now(mode, cx));
            cx.update_window(handle, |_, window, cx| {
                window.render_frame(cx);
                let glass = window.painted_backdrop_blurs();
                assert_eq!(glass.len(), 2);
                assert_eq!(glass[0].bounds.size, glass[1].bounds.size);
                assert_eq!(glass[0].tint, glass[1].tint);
                assert_eq!(glass[0].tint, cx.theme().colors.danger.opacity(0.7));
                assert!(glass[0].blur_radius.0 > 0.);
            })
            .unwrap();
        }
    }

    #[test]
    fn unread_label_preserves_small_counts_and_caps_large_counts() {
        assert_eq!(label(0), "0");
        assert_eq!(label(3), "3");
        assert_eq!(label(99), "99");
        assert_eq!(label(100), "99+");
        assert_eq!(label(usize::MAX), "99+");
    }
}
