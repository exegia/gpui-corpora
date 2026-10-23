use std::sync::{Arc, LazyLock};

use corpora_ui::ely::theme::{ActiveTheme, Radius, TextSize};
use gpui::{
    App, FontWeight, Image, ImageFormat, IntoElement, MouseButton, ObjectFit, RenderOnce, Role,
    SharedString, Window, div, img, prelude::*, rems,
};

use crate::app::{self, theme::ThemeMode};

static LIGHT: LazyLock<Arc<Image>> =
    LazyLock::new(|| preview(include_bytes!("../assets/appearance/appearance_light.png")));
static DARK: LazyLock<Arc<Image>> =
    LazyLock::new(|| preview(include_bytes!("../assets/appearance/appearance_dark.png")));
static SYSTEM: LazyLock<Arc<Image>> =
    LazyLock::new(|| preview(include_bytes!("../assets/appearance/appearance_system.png")));

fn preview(bytes: &[u8]) -> Arc<Image> {
    Arc::new(Image::from_bytes(ImageFormat::Png, bytes.to_vec()))
}

pub(super) fn theme_picker(cx: &App) -> impl IntoElement + use<> {
    ThemePicker {
        selected: app::theme::current_mode(cx),
    }
}

#[derive(IntoElement)]
struct ThemePicker {
    selected: ThemeMode,
}

impl RenderOnce for ThemePicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let options = [
            (
                "appearance-system",
                ThemeMode::System,
                "menu_appearance_system",
                &SYSTEM,
            ),
            (
                "appearance-light",
                ThemeMode::Light,
                "menu_appearance_light",
                &LIGHT,
            ),
            (
                "appearance-dark",
                ThemeMode::Dark,
                "menu_appearance_dark",
                &DARK,
            ),
        ];
        let handles: Vec<_> = options
            .iter()
            .map(|(id, ..)| {
                window
                    .use_keyed_state(SharedString::from(format!("{id}-focus")), cx, |_, cx| {
                        cx.focus_handle()
                    })
                    .read(cx)
                    .clone()
                    .tab_stop(true)
            })
            .collect();
        let theme = cx.theme();
        let colors = &theme.colors;
        div()
            .id("settings-theme-picker")
            .role(Role::RadioGroup)
            .aria_label(crate::i18n::localize("settings_appearance"))
            .flex()
            .flex_wrap()
            .items_start()
            .gap_4()
            .children(
                options
                    .iter()
                    .enumerate()
                    .map(|(index, &(id, mode, label, image))| {
                        let selected = self.selected == mode;
                        let focus = handles[index].clone();
                        let previous =
                            (options[(index + 2) % 3].1, handles[(index + 2) % 3].clone());
                        let next = (options[(index + 1) % 3].1, handles[(index + 1) % 3].clone());
                        let label = crate::i18n::localize(label);
                        div()
                            .id(id)
                            .role(Role::RadioButton)
                            .aria_label(label.clone())
                            .aria_toggled(selected.into())
                            .track_focus(&focus)
                            .w(rems(9.0))
                            .flex_none()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1p5()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.prevent_default()
                            })
                            .on_click(move |_, window, cx| {
                                window.focus(&focus, cx);
                                if !selected {
                                    app::set_theme_mode(mode, cx);
                                }
                            })
                            .on_key_down(move |event, window, cx| {
                                let choice = match event.keystroke.key.as_str() {
                                    "left" | "up" => &previous,
                                    "right" | "down" => &next,
                                    _ => return,
                                };
                                window.prevent_default();
                                cx.stop_propagation();
                                window.focus(&choice.1, cx);
                                app::set_theme_mode(choice.0, cx);
                            })
                            .child(
                                div()
                                    .id(SharedString::from(format!("{id}-preview")))
                                    .w_full()
                                    .h(rems(6.0))
                                    .border_2()
                                    .border_color(if selected {
                                        colors.accent
                                    } else {
                                        colors.border
                                    })
                                    .overflow_hidden()
                                    .rounded(theme.radius(Radius::Xl))
                                    .when(!selected, |preview| {
                                        preview
                                            .hover(|style| style.border_color(colors.border_strong))
                                    })
                                    .child(
                                        img(Arc::clone(image))
                                            .size_full()
                                            .rounded(theme.radius(Radius::Xl))
                                            .object_fit(ObjectFit::Cover),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(theme.text_size(TextSize::Md))
                                    .font_weight(if selected {
                                        FontWeight::SEMIBOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .text_color(if selected { colors.fg } else { colors.fg_muted })
                                    .child(label),
                            )
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_previews_decode_at_matching_sizes_with_distinct_appearances() {
        let decode = |image: &Arc<Image>| {
            image::load_from_memory_with_format(image.bytes(), image::ImageFormat::Png)
                .unwrap()
                .into_rgba8()
        };
        let light = decode(&LIGHT);
        let dark = decode(&DARK);
        let system = decode(&SYSTEM);
        assert!(light.width() > 0 && light.height() > 0);
        assert_eq!(light.dimensions(), dark.dimensions());
        assert_eq!(light.dimensions(), system.dimensions());
        assert_ne!(light.as_raw(), dark.as_raw());
        assert_ne!(system.as_raw(), light.as_raw());
        assert_ne!(system.as_raw(), dark.as_raw());
    }
}
