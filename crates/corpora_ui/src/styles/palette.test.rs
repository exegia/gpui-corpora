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
    assert_eq!(light.accent, resolve("amber.500"));
    assert_eq!(light.bg, resolve("slate.100"));
    assert_eq!(light.fg, resolve("neutral.950"));

    let dark = build(Mode::Dark);
    assert_eq!(dark.accent, resolve("amber.300"));
    assert_eq!(dark.syntax.keyword, resolve("#c28b12"));
}
