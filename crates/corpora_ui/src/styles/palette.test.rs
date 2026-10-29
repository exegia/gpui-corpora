use super::*;

#[test]
fn resolves_named_and_hex_colors() {
    assert_eq!(resolve("white"), rgb(0xffffff).into());
    assert_eq!(resolve("black"), rgb(0x000000).into());
    assert_eq!(resolve("#1a2b3c"), rgb(0x1a2b3c).into());
}

#[test]
fn resolves_a_scale_step_through_oklch() {
    // amber.500 is oklch(76.9% 0.188 70.08) in palette.toml.
    assert_eq!(resolve("amber.500"), oklch(0.769, 0.188, 70.08));
}

#[test]
fn build_maps_roles_onto_palette_fields() {
    let light = build(Mode::Light);
    assert_eq!(light.accent, resolve("indigo.600"));
    assert_eq!(light.bg, resolve("slate.200"));
    assert_eq!(light.fg, resolve("neutral.950"));

    let dark = build(Mode::Dark);
    assert_eq!(dark.accent, resolve("indigo.300"));
    assert_eq!(dark.syntax.keyword, resolve("#c28b12"));
}

#[test]
fn every_palette_color_uses_the_theme_document() {
    let base = parsed();
    let mut custom = Parsed {
        scales: base.scales.clone(),
        named: base.named.clone(),
        theme: toml::from_str(include_str!("../themes/default.toml")).unwrap(),
    };
    for role in custom.theme.colors.values_mut() {
        role.light = "#123456".into();
        role.dark = "#654321".into();
    }
    for (mode, reference) in [(Mode::Light, "#123456"), (Mode::Dark, "#654321")] {
        let palette = build_from(&custom, mode);
        let expected = parse_color(reference);
        for name in Palette::NAMES {
            let actual = palette.token(name);
            assert_eq!(actual.to_rgb().r, expected.to_rgb().r, "{name}");
            assert_eq!(actual.to_rgb().g, expected.to_rgb().g, "{name}");
            assert_eq!(actual.to_rgb().b, expected.to_rgb().b, "{name}");
        }
    }
}
