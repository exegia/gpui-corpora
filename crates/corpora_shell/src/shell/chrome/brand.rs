#[allow(unused_imports)]
use super::*;
use std::borrow::Cow;
use std::sync::{Arc, LazyLock};

use ely_gpui_component::theme::{ActiveTheme, TextSize};
use gpui::{App, Image, ImageFormat, Img, ObjectFit, div, img, px};
use gpui_kit::base::StyledExt;

static LIGHT: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Png,
        include_bytes!("../assets/brand/light.png").to_vec(),
    ))
});
static DARK: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Png,
        include_bytes!("../assets/brand/dark.png").to_vec(),
    ))
});

/// The logo image for the current theme's appearance.
pub fn logo_image(cx: &App) -> Arc<Image> {
    Arc::clone(if cx.theme().is_dark() { &DARK } else { &LIGHT })
}

pub fn logo(cx: &App) -> Img {
    img(logo_image(cx))
}

/// Register the bundled Caudex regular and bold fonts before opening windows.
pub fn register_brand_fonts(cx: &App) -> anyhow::Result<()> {
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(include_bytes!("../assets/fonts/caudex/Caudex-Regular.ttf").as_slice()),
        Cow::Borrowed(include_bytes!("../assets/fonts/caudex/Caudex-Bold.ttf").as_slice()),
    ])
}

pub(super) fn brand(cx: &App) -> impl IntoElement + use<> {
    let theme = cx.theme();
    let image = logo(cx).size(px(18.0)).object_fit(ObjectFit::Contain);
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap_1()
        .text_color(theme.colors.fg)
        .child(image)
        .child(
            div()
                .font_family("Caudex")
                .text_size(theme.text_size(TextSize::Lg))
                .font_medium()
                .child("Corpora"),
        )
}
