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
