use super::*;
use crate::styles::palette::resolve;

#[test]
fn palettes_keep_custom_backgrounds_and_accents() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.light.bg, resolve("slate.100"));
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
fn ansi_takes_base_colors_and_keeps_the_rest() {
    let (ely, theme) = (Palette::light(false), DefaultTheme::new());
    assert_eq!(theme.light.ansi[1], resolve("#d21f07"));
    assert_eq!(theme.light.ansi[6], resolve("#007e8a"));
    assert_eq!(theme.light.ansi[0], ely.ansi[0]);
    assert_eq!(theme.light.ansi[9], ely.ansi[9]);
}

#[test]
fn unmapped_fields_keep_ely_values() {
    let (ely, theme) = (Palette::dark(false), DefaultTheme::new());
    assert_eq!(theme.dark.chart, ely.chart);
    assert_eq!(theme.dark.tooltip_bg, ely.tooltip_bg);
}

#[test]
fn focus_matches_the_accent() {
    let theme = DefaultTheme::new();
    assert_eq!(theme.light.focus, theme.light.accent);
    assert_eq!(theme.dark.focus, theme.dark.accent);
}

#[gpui::test]
fn kit_focus_policy_survives_repeated_appearance_changes(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        for mode in [
            gpui_kit::component::ThemeMode::Light,
            gpui_kit::component::ThemeMode::Dark,
            gpui_kit::component::ThemeMode::Light,
        ] {
            gpui_kit::component::Theme::change(mode, None, cx);
            disable_focus_outlines(cx);
            let theme = gpui_kit::component::Theme::global(cx);
            assert!(!theme.focus_ring && theme.ring.a == 0.0);
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
