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
