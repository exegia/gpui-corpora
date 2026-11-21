use gpui::{Background, Hsla, black, white};

use super::*;

fn lightness(color: Hsla) -> f32 {
    to_oklch(color).l
}

fn close(a: Hsla, b: Hsla) -> bool {
    let (a, b) = (a.to_rgb(), b.to_rgb());
    [(a.r, b.r), (a.g, b.g), (a.b, b.b), (a.a, b.a)]
        .iter()
        .all(|(x, y)| (x - y).abs() < 0.005)
}

#[test]
fn lighten_and_darken_reach_white_and_black_at_one() {
    let amber = Hsla::from(Hex(0xfe9a00));
    assert!(close(amber.lighten(1.0), white()));
    assert!(close(amber.darken(1.0), black()));
    assert!(close(amber.lighten(0.0), amber));
}

#[test]
fn adjustments_keep_alpha_and_work_on_any_color_type() {
    let translucent = Hex(0x0060de).into();
    let translucent = Hsla::alpha(&translucent, 0.4);
    assert_eq!(translucent.lighten(0.3).a, 0.4);
    let hex = Hex(0x808080).darken(0.5);
    assert!(Hsla::from(hex).to_rgb().r < 0.5);
    let rgba = Hsla::from(Hex(0x808080)).to_rgb().lighten(0.5);
    assert!(rgba.r > 0.5);
}

#[test]
fn saturated_tints_stay_in_gamut() {
    let tint = Hsla::from(Hex(0xff2056)).lighten(0.6).to_rgb();
    for channel in [tint.r, tint.g, tint.b] {
        assert!((0.0..=1.0).contains(&channel));
    }
    assert!(tint.r > tint.g, "the tint stays rose");
}

#[test]
fn scale_keeps_its_base_at_500_and_orders_by_lightness() {
    let base = Hsla::from(Hex(0x30d158));
    let scale = convert(base);
    assert_eq!(scale[500], base);
    let steps: Vec<f32> = (1..=9).map(|step| lightness(scale[step * 100])).collect();
    assert!(steps.windows(2).all(|pair| pair[0] > pair[1]), "{steps:?}");
}

#[test]
fn tight_scales_move_less_than_wide_ones() {
    let bg = Hsla::from(Hex(0x131313));
    let (tight, wide) = (Scale::new(bg, Spread::TIGHT), convert(bg));
    assert!(lightness(tight[100]) < lightness(wide[100]));
    assert!(lightness(tight[900]) > lightness(wide[900]));
}

#[test]
#[should_panic(expected = "scale steps run 100 to 900")]
fn scale_rejects_steps_between_hundreds() {
    let _ = convert(Hex(0x000000))[150];
}

#[test]
fn contrast_matches_wcag_extremes() {
    assert!((black().contrast(white()) - 21.0).abs() < 0.01);
    assert!((white().contrast(white()) - 1.0).abs() < 0.01);
}

#[test]
fn hex_round_trips_through_hsla() {
    assert_eq!(Hex::from(Hsla::from(Hex(0xfe9a00))), Hex(0xfe9a00));
}

#[test]
fn gradient_runs_from_the_color_toward_its_target() {
    let base = Hsla::from(Hex(0x131313));
    let expected = linear_gradient(
        90.0,
        linear_color_stop(base, 0.0),
        linear_color_stop(base.lighten(0.2), 1.0),
    );
    let gradient: Background = base.to_gradient(Toward::Lighter(0.2), 90.0);
    assert_eq!(gradient, expected);
}

// --- palette parsing, `palette()` and `theme()` ---

fn ramp_toml(name: &str, skip: Option<u16>) -> String {
    let mut out = format!("[colors.{name}]\n");
    for (i, step) in RAMP_STEPS.iter().enumerate() {
        if Some(*step) != skip {
            out += &format!("{step} = \"oklch({}% 0.02 250)\"\n", 98 - i * 8);
        }
    }
    out
}

fn minimal(extra: &str) -> String {
    format!("[colors]\nblack = \"#000\"\nwhite = \"#fff\"\n{extra}")
}

#[test]
fn palette_reads_oklch_values_from_palette_toml() {
    // gray.500 is oklch(55.1% 0.027 264.364) in palette.toml.
    assert!(close(palette("gray", 500), oklch(0.551, 0.027, 264.364)));
    assert!(close(palette("red", 50), oklch(0.971, 0.013, 17.38)));
    assert!(close(palette("taupe", 950), oklch(0.147, 0.004, 49.3)));
}

#[test]
fn palette_resolves_black_and_white_regardless_of_scale() {
    assert!(close(palette("black", 0), black()));
    assert!(close(palette("white", 500), white()));
}

#[test]
fn palette_ramps_get_darker_from_50_to_950() {
    for name in ["gray", "indigo", "red", "neutral", "taupe"] {
        let lightness: Vec<f32> = RAMP_STEPS
            .iter()
            .map(|step| lightness(palette(name, *step)))
            .collect();
        assert!(
            lightness.windows(2).all(|pair| pair[0] > pair[1]),
            "{name} is not strictly darkening: {lightness:?}"
        );
    }
}

#[test]
#[should_panic(expected = "no palette color nope.500")]
fn palette_panics_on_unknown_name() {
    palette("nope", 500);
}

#[test]
#[should_panic(expected = "no palette color gray.550")]
fn palette_panics_on_unknown_step() {
    palette("gray", 550);
}

#[test]
fn parser_loads_every_ramp_in_the_shipped_file() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/themes/palette.toml");
    let parsed = parser(path).expect("palette.toml parses");
    assert_eq!(parsed.name, "Palette");
    assert!(parsed.ramp_names().count() >= 20);
    for name in parsed.ramp_names() {
        for step in RAMP_STEPS {
            assert!(parsed.color(name, step).is_some(), "{name}.{step}");
        }
    }
    assert_eq!(parsed["gray"][500], palette("gray", 500));
}

#[test]
fn parser_reports_a_missing_file_as_io() {
    assert!(matches!(
        parser("/no/such/palette.toml"),
        Err(PaletteError::Io(_))
    ));
}

#[test]
fn parse_palette_rejects_bad_input_with_a_useful_error() {
    assert!(matches!(
        parse_palette("not = [valid"),
        Err(PaletteError::Toml(_))
    ));

    let missing_step = parse_palette(&minimal(&ramp_toml("gray", Some(950)))).unwrap_err();
    assert!(
        missing_step
            .to_string()
            .contains("gray is missing step 950")
    );

    let bad_color = parse_palette(&minimal("[colors.gray]\n50 = \"banana\"\n")).unwrap_err();
    assert!(matches!(bad_color, PaletteError::Schema(_)));

    let no_black = parse_palette("[colors]\nwhite = \"#fff\"\n").unwrap_err();
    assert!(no_black.to_string().contains("colors.black"));

    let stray = parse_palette(&minimal("mystery = \"#123456\"\n")).unwrap_err();
    assert!(stray.to_string().contains("mystery"));
}

#[test]
fn parse_color_handles_hex_shorthand_and_oklch_alpha() {
    assert!(close(parse_color("#fff").unwrap(), white()));
    assert!(close(parse_color("#000000").unwrap(), black()));
    assert!(close(parse_color("#f00").unwrap(), Hex(0xff0000).into()));
    let translucent = parse_color("oklch(50% 0.1 200 / 40%)").unwrap();
    assert!((translucent.a - 0.4).abs() < 1e-4);
    assert!(parse_color("oklch(50% 0.1)").is_err());
    assert!(parse_color("rgb(1 2 3)").is_err());
}

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const VARIANTS: [Variant; 8] = [
    Variant::Accent,
    Variant::Filled,
    Variant::Outline,
    Variant::Subtle,
    Variant::Danger,
    Variant::Info,
    Variant::Warning,
    Variant::Success,
];
const ROLES: [Role; 6] = [
    Role::Foreground,
    Role::Background,
    Role::Surface,
    Role::Elevation,
    Role::Outline,
    Role::Shadow,
];

#[test]
fn theme_resolves_every_variant_role_and_mode() {
    for variant in VARIANTS {
        for role in ROLES {
            for mode in MODES {
                let color = theme(variant, role, mode);
                assert!(
                    (0.0..=1.0).contains(&color.a),
                    "{variant:?}/{role:?}/{mode:?}"
                );
            }
        }
    }
}

#[test]
fn theme_solid_variants_map_to_palette_steps_per_mode() {
    let cases = [
        (Variant::Danger, "red"),
        (Variant::Info, "sky"),
        (Variant::Warning, "amber"),
        (Variant::Success, "green"),
        (Variant::Accent, "indigo"),
    ];
    for (variant, ramp) in cases {
        assert_eq!(
            theme(variant, Role::Background, Mode::Light),
            palette(ramp, 600)
        );
        assert_eq!(
            theme(variant, Role::Background, Mode::Dark),
            palette(ramp, 500)
        );
        assert_eq!(
            theme(variant, Role::Foreground, Mode::Light),
            palette("white", 0)
        );
        assert_eq!(
            theme(variant, Role::Foreground, Mode::Dark),
            palette(ramp, 950)
        );
    }
}

#[test]
fn theme_outline_is_transparent_and_tinted_is_pale() {
    for mode in MODES {
        assert_eq!(theme(Variant::Outline, Role::Background, mode).a, 0.0);
    }
    assert!(
        lightness(theme(Variant::Subtle, Role::Background, Mode::Light))
            > lightness(theme(Variant::Subtle, Role::Background, Mode::Dark))
    );
}

#[test]
fn theme_shadow_is_translucent_black_and_deeper_in_dark() {
    let light = theme(Variant::Accent, Role::Shadow, Mode::Light);
    let dark = theme(Variant::Accent, Role::Shadow, Mode::Dark);
    assert!(light.a > 0.0 && light.a < dark.a && dark.a < 1.0);
    assert!(lightness(light) < 0.01 && lightness(dark) < 0.01);
}

#[test]
fn theme_text_is_readable_on_solid_fills() {
    // Amber is excluded: white on amber.600 is a known weak pairing.
    for variant in [
        Variant::Accent,
        Variant::Danger,
        Variant::Success,
        Variant::Info,
    ] {
        for mode in MODES {
            let text = theme(variant, Role::Foreground, mode);
            let fill = theme(variant, Role::Background, mode);
            assert!(
                text.contrast(fill) >= 3.0,
                "{variant:?}/{mode:?} contrast {}",
                text.contrast(fill)
            );
        }
    }
}

#[test]
fn semantics_load_matches_theme_fn() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/themes/palette.toml");
    let semantics = Semantics::load(path).expect("loads");
    for variant in VARIANTS {
        for role in ROLES {
            for mode in MODES {
                assert_eq!(
                    semantics.resolve(variant, mode).role_color(role),
                    theme(variant, role, mode)
                );
            }
        }
    }
}

#[test]
fn semantics_reports_a_missing_ramp() {
    let sparse = parse_palette(&minimal(&ramp_toml("gray", None))).unwrap();
    let error = Semantics::from_palette(&sparse).unwrap_err();
    assert!(error.to_string().contains("needs color ramp"));
}

#[test]
fn bezel_tones_are_white_and_black_with_per_mode_alpha() {
    let (lit, dim) = bezel_tones(Mode::Light);
    assert!(close(Hsla { a: 1.0, ..lit }, white()) && (lit.a - 0.32).abs() < 1e-6);
    assert!(close(Hsla { a: 1.0, ..dim }, black()) && (dim.a - 0.22).abs() < 1e-6);
    let (lit, dim) = bezel_tones(Mode::Dark);
    assert!((lit.a - 0.24).abs() < 1e-6 && (dim.a - 0.3).abs() < 1e-6);
}

#[test]
fn bezel_paints_a_lit_top_and_a_dim_bottom_inset() {
    let (lit, dim) = bezel_tones(Mode::Light);
    let shadows = bezel(lit, dim).shadows();
    assert_eq!(shadows.len(), 2);
    assert!(shadows.iter().all(|shadow| shadow.inset));
    let offsets: Vec<f32> = shadows.iter().map(|s| f32::from(s.offset.y)).collect();
    assert!(
        offsets.contains(&1.0) && offsets.contains(&-1.0),
        "{offsets:?}"
    );
}
