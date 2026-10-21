use std::sync::Arc;

use corpora_ui::ely::{
    motion::{self, ProgressBar},
    primitives::FocusScope,
    theme::{ActiveTheme, ControlSize, TextSize},
    typography::{Caption, Title},
};
use gpui_kit::{
    Animation, AnimationExt, App, AppContext, Bounds, Context, FocusHandle, Image, ObjectFit,
    ParentElement, Render, Styled, StyledImage, Window, WindowBounds, WindowHandle, WindowOptions,
    div, img, relative,
};

use corpora_shell::shell::logo_image;

pub(super) struct SplashArtwork {
    image: Arc<Image>,
    focus: FocusHandle,
}

impl Render for SplashArtwork {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl gpui_kit::IntoElement {
        let theme = cx.theme();
        let enter = motion::duration(motion::SLOW, cx);
        FocusScope::new(&self.focus)
            .root()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .px_10()
            .bg(theme.colors.bg)
            .text_color(theme.colors.fg)
            .font_family(theme.font_family.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .child(
                        img(self.image.clone())
                            .size(theme.control_height(ControlSize::Lg) * 2.0)
                            .object_fit(ObjectFit::Contain),
                    )
                    .child(
                        div()
                            .pt_2()
                            .font_family("Caudex")
                            .child(Title::new("Corpora")),
                    )
                    .child(Caption::new(
                        "Read deeply. Research openly. Discover together.",
                    ))
                    .child(Caption::new(concat!("v", env!("CARGO_PKG_VERSION"))))
                    .with_animation(
                        "splash-in",
                        Animation::new(enter).with_easing(motion::ease_out_cubic),
                        |block, t| block.opacity(t).mt(motion::NUDGE * 2.0 * (1.0 - t)),
                    ),
            )
            .child(
                div()
                    .w(relative(0.6))
                    .pt_8()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w_full()
                            .child(ProgressBar::indeterminate("splash-progress")),
                    )
                    .child(
                        div()
                            .text_size(theme.text_size(TextSize::Xs))
                            .text_color(theme.colors.fg_subtle)
                            .child("Opening workspace…"),
                    ),
            )
    }
}

pub(super) fn open(cx: &mut App) -> anyhow::Result<WindowHandle<SplashArtwork>> {
    let image = logo_image(cx);
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            cx.theme().splash_window(),
            cx,
        ))),
        titlebar: None,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    cx.open_window(options, |window, cx| {
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        cx.new(|_| SplashArtwork { image, focus })
    })
}
