use corpora_ui::ely::theme::{ActiveTheme, Palette, Radius, TextSize};
use gpui::{
    App, Bounds, Corners, FontWeight, IntoElement, MouseButton, Pixels, RenderOnce, Role,
    SharedString, Window, canvas, div, fill, point, prelude::*, px, rems, size,
};

use crate::app::{self, theme::ThemeMode};

fn preview_half(
    colors: &Palette,
    bounds: Bounds<Pixels>,
    left: bool,
    right: bool,
    window: &mut Window,
) {
    let corners = Corners {
        top_left: px(if left { 10. } else { 0. }),
        bottom_left: px(if left { 10. } else { 0. }),
        top_right: px(if right { 10. } else { 0. }),
        bottom_right: px(if right { 10. } else { 0. }),
    };
    window.paint_quad(fill(bounds, colors.bg).corner_radii(corners));
    let header = Bounds {
        size: size(bounds.size.width, px(12.)),
        ..bounds
    };
    window.paint_quad(fill(header, colors.surface).corner_radii(Corners {
        bottom_left: px(0.),
        bottom_right: px(0.),
        ..corners
    }));
    let mut rect = |x, y, width, height, color| {
        window.paint_quad(
            fill(
                Bounds {
                    origin: bounds.origin + point(px(x), px(y)),
                    size: size(px(width), px(height)),
                },
                color,
            )
            .corner_radii(px(3.)),
        );
    };
    let width = f32::from(bounds.size.width);
    let height = f32::from(bounds.size.height);
    rect(4., 16., 20., height - 20., colors.sunken);
    rect(8., 20., 12., 4., colors.accent);
    rect(8., 28., 12., 4., colors.fg_muted);
    rect(28., 16., width - 32., 12., colors.surface);
    rect(28., 32., (width - 36.).min(32.), 4., colors.fg);
    rect(28., 40., (width - 36.).min(24.), 4., colors.fg_muted);
    rect(28., 48., width - 32., 16., colors.surface);
}

fn preview(mode: ThemeMode) -> impl IntoElement {
    let theme = corpora_ui::styles::tokens::DefaultTheme::new();
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| match mode {
            ThemeMode::Light => preview_half(&theme.light, bounds, true, true, window),
            ThemeMode::Dark => preview_half(&theme.dark, bounds, true, true, window),
            ThemeMode::System => {
                let half = bounds.size.width / 2.;
                let left = Bounds {
                    size: size(half, bounds.size.height),
                    ..bounds
                };
                let right = Bounds {
                    origin: bounds.origin + point(half, px(0.)),
                    size: left.size,
                };
                preview_half(&theme.light, left, true, false, window);
                preview_half(&theme.dark, right, false, true, window);
            }
        },
    )
    .size_full()
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
            ),
            (
                "appearance-light",
                ThemeMode::Light,
                "menu_appearance_light",
            ),
            ("appearance-dark", ThemeMode::Dark, "menu_appearance_dark"),
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
                    .map(|(index, &(id, mode, label))| {
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
                                    .child(preview(mode)),
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
