use super::*;

#[test]
fn palettes_carry_the_source_theme_colors() {
    let theme = MacosClassic::new();
    assert_eq!(theme.light.bg, hex(0xf9f9f9));
    assert_eq!(theme.light.accent, hex(0xfe9a00));
    assert_eq!(theme.dark.accent, hex(0xffd230));
    assert_eq!(theme.dark.bg, hex(0x131313));
    assert_eq!(theme.dark.syntax.keyword, hex(0xc28b12));
}

#[test]
fn palette_picks_the_mode() {
    let theme = MacosClassic::new();
    assert_eq!(theme.palette(Mode::Light), &theme.light);
    assert_eq!(theme.palette(Mode::Dark), &theme.dark);
}

#[test]
fn ansi_takes_base_colors_and_keeps_the_rest() {
    let (ely, theme) = (Palette::light(false), MacosClassic::new());
    assert_eq!(theme.light.ansi[1], hex(0xd21f07));
    assert_eq!(theme.light.ansi[6], hex(0x007e8a));
    assert_eq!(theme.light.ansi[0], ely.ansi[0]);
    assert_eq!(theme.light.ansi[9], ely.ansi[9]);
}

#[test]
fn unmapped_fields_keep_ely_values() {
    let (ely, theme) = (Palette::dark(false), MacosClassic::new());
    assert_eq!(theme.dark.chart, ely.chart);
    assert_eq!(theme.dark.tooltip_bg, ely.tooltip_bg);
}

#[test]
fn neither_palette_paints_an_accent_focus_outline() {
    let theme = MacosClassic::new();
    assert_eq!((theme.light.focus.a, theme.dark.focus.a), (0.0, 0.0));
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

#[test]
fn shades_derive_from_their_base_colors() {
    let theme = MacosClassic::new();
    assert_eq!(theme.light.fg_muted, convert(theme.light.fg)[300]);
    assert_eq!(theme.dark.fg_muted, convert(theme.dark.fg)[700]);
    assert_eq!(
        theme.light.sunken,
        Scale::new(theme.light.bg, Spread::TIGHT)[600]
    );
    assert_eq!(theme.dark.on_accent, convert(theme.dark.accent)[900]);
}

#[test]
fn derived_text_stays_readable() {
    use crate::styles::utils::ColorExt as _;
    for (mode, palette) in [
        ("light", MacosClassic::new().light),
        ("dark", MacosClassic::new().dark),
    ] {
        for (name, color, surface) in [
            ("fg_muted", palette.fg_muted, palette.bg),
            ("link", palette.link, palette.bg),
            ("on_accent", palette.on_accent, palette.accent),
            ("fg on selection", palette.fg, palette.selection),
        ] {
            let ratio = color.contrast(surface);
            assert!(ratio >= 4.5, "{mode} {name} is {ratio:.2}:1");
        }
    }
}
