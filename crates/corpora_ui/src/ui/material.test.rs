use gpui::{Hsla, px};
use gpui_agentation::ely::theme::Mode;

use super::{GlassOverrides, GlassSpec};

#[test]
fn no_overrides_returns_the_preset_unchanged() {
    for mode in [Mode::Light, Mode::Dark] {
        let plain = GlassSpec::secondary(mode, None);
        let empty = GlassSpec::secondary(mode, Some(GlassOverrides::default()));
        assert_eq!(plain.lens, empty.lens);
        assert_eq!(plain.edge, empty.edge);
        assert_eq!(plain.tint, empty.tint);
    }
}

#[test]
fn overrides_replace_only_the_fields_they_set() {
    let base = GlassSpec::accent(Mode::Dark, None);
    let spec = GlassSpec::accent(
        Mode::Dark,
        Some(GlassOverrides {
            edge: Some(0.5),
            edge_width: Some(px(2.)),
            ..GlassOverrides::default()
        }),
    );

    assert_eq!(spec.edge, 0.5);
    assert_eq!(spec.edge_width, px(2.));
    assert_eq!(spec.blur_radius, base.blur_radius);
    assert_eq!(spec.lens, base.lens);
    assert_eq!(spec.saturation, base.saturation);
    assert_eq!(spec.tint, base.tint);
}

#[test]
fn tint_opacity_adds_the_semantic_tint_when_the_preset_has_none() {
    assert!(GlassSpec::secondary(Mode::Light, None).tint.is_none());

    let spec = GlassSpec::secondary(
        Mode::Light,
        Some(GlassOverrides {
            tint_opacity: Some(0.25),
            ..GlassOverrides::default()
        }),
    );
    assert_eq!(spec.tint.expect("tint is added").a, 0.25);
}

#[test]
fn tint_override_wins_and_opacity_applies_to_it() {
    let red = Hsla::red();
    let spec = GlassSpec::destructive(
        Mode::Dark,
        Some(GlassOverrides {
            tint: Some(red),
            tint_opacity: Some(0.4),
            ..GlassOverrides::default()
        }),
    );
    let tint = spec.tint.expect("tint override applies");
    assert_eq!(tint.h, red.h);
    assert_eq!(tint.a, 0.4);
}
