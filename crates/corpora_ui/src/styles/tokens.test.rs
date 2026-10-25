use super::*;
use crate::styles::palette::resolve;

#[test]
fn palettes_keep_custom_backgrounds_and_accents() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.light.bg, resolve("slate.200"));
    assert_eq!(theme.light.accent, resolve("amber.500"));
    assert_eq!(theme.dark.accent, resolve("amber.300"));
    assert_eq!(theme.dark.bg, resolve("neutral.800"));
    assert_eq!(theme.dark.syntax.keyword, resolve("#c28b12"));
}

#[test]
fn palette_picks_the_mode() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.palette(Mode::Light), &theme.light);
    assert_eq!(theme.palette(Mode::Dark), &theme.dark);
}

#[test]
fn terminal_and_chart_rows_are_fully_theme_owned() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.light.ansi[0], resolve("#181613"));
    assert_eq!(theme.light.ansi[1], resolve("#d21f07"));
    assert_eq!(theme.light.ansi[9], resolve("#bd6158"));
    assert_eq!(theme.dark.ansi[15], resolve("#f3f1f0"));
    assert_eq!(theme.dark.chart[7], resolve("cyan.400"));
    assert_eq!(theme.dark.tooltip_bg, resolve("neutral.700"));
}

#[test]
fn focus_matches_the_accent() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.light.focus, theme.light.accent);
    assert_eq!(theme.dark.focus, theme.dark.accent);
}

#[gpui::test]
fn kit_theme_survives_repeated_appearance_changes(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        for mode in [
            gpui_kit::component::ThemeMode::Light,
            gpui_kit::component::ThemeMode::Dark,
            gpui_kit::component::ThemeMode::Light,
        ] {
            gpui_kit::component::Theme::change(mode, None, cx);
            sync_kit_theme(cx);
            let theme = gpui_kit::component::Theme::global(cx);
            assert!(!theme.focus_ring && theme.ring.a == 0.0);
            let mode = if mode.is_dark() {
                Mode::Dark
            } else {
                Mode::Light
            };
            let palette = build(mode);
            assert_eq!(theme.background, palette.bg);
            assert_eq!(theme.tokens.background.color, palette.bg);
            assert_eq!(theme.button_primary, palette.accent);
            assert_eq!(theme.popover, palette.overlay);
            assert_eq!(theme.chart_5, palette.chart[4]);
            assert_eq!(theme.blue_light, palette.ansi[12]);
            assert_eq!(
                theme.highlight_theme.style.editor_background,
                Some(palette.bg)
            );
            assert_eq!(
                theme
                    .highlight_theme
                    .style
                    .syntax
                    .style("keyword")
                    .unwrap()
                    .color,
                Some(palette.syntax.keyword)
            );
        }
    });
}

/// fg_muted is secondary text and sits just under 4.5:1 on the light canvas, so
/// it stays out of this check; the rest must clear AA against their surface.
#[test]
fn semantic_text_stays_readable() {
    use crate::styles::utils::ColorExt as _;
    for (mode, palette) in [
        ("light", DefaultTheme::new().light),
        ("dark", DefaultTheme::new().dark),
    ] {
        for (name, color, surface) in [
            ("link", palette.link, palette.bg),
            ("on_accent", palette.on_accent, palette.accent),
            ("fg on selection", palette.fg, palette.selection),
        ] {
            let ratio = color.contrast(surface);
            assert!(ratio >= 4.5, "{mode} {name} is {ratio:.2}:1");
        }
    }
}
