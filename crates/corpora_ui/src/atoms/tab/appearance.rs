use crate::ely::theme::{Mode, Palette, Theme};
use crate::ui::material::{GlassOverrides, GlassSpec};
use gpui::{Hsla, px};

pub(super) const VERTICAL_TAB_RADIUS: f32 = 14.;
pub(super) const VERTICAL_TAB_LABEL_SIZE: f32 = 9.;
pub(super) const HORIZONTAL_TAB_GAP: f32 = 6.;

/// Workspace tab colors are shared by both layouts.
pub(super) fn content_color(colors: &Palette, active: bool) -> Hsla {
    if active {
        colors.fg
    } else {
        colors.fg.opacity(0.6)
    }
}

/// Glass behind a horizontal tab. `strength` runs from 0 (no glass) to 1
/// (selected) and scales the tint and rim, so hover shows a quieter version
/// of the selected material.
pub(super) fn glass_spec(theme: &Theme, strength: f32) -> GlassSpec {
    let mode = if theme.is_dark() {
        Mode::Dark
    } else {
        Mode::Light
    };
    let tint = theme.colors.fg_muted.opacity(0.10);
    let mut spec = GlassSpec::secondary(
        mode,
        Some(GlassOverrides {
            blur_radius: Some(px(5.5)),
            lens: Some(px(24.0)),
            magnify: Some(1.20),
            dispersion: Some(0.180),
            gain_light: Some(1.150),
            gain_dark: Some(1.150),
            saturation: Some(1.),
            tint: Some(tint),
            edge: Some(0.26),
            edge_width: Some(px(1.)),
            edge_aa: Some(px(0.5)),
            ..GlassOverrides::default()
        }),
    );
    spec.tint = spec.tint.map(|tint| Hsla {
        a: tint.a * strength,
        ..tint
    });
    spec.edge *= strength;
    spec
}
