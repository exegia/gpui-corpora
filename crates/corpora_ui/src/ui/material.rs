//! Glass surface wrapper adapted from Bezel's MIT-licensed `ui::surface` module.

use gpui::{
    AbsoluteLength, AnyElement, App, Bounds, Corners, Element, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, Pixels, Styled, Window, px,
};
use gpui_agentation::ely::theme::{Mode, Palette};
use crate::styles::{tokens::DefaultTheme};

use super::glass;

pub use super::glass::GlassSpec;

/// Per-field overrides layered over a preset. `None` keeps the preset's value.
#[derive(Clone, Copy, Debug, Default)]
pub struct GlassOverrides {
    pub blur_radius: Option<Pixels>,
    pub lens: Option<Pixels>,
    pub reach: Option<Pixels>,
    pub magnify: Option<f32>,
    pub dispersion: Option<f32>,
    pub gain_light: Option<f32>,
    pub gain_dark: Option<f32>,
    pub saturation: Option<f32>,
    pub edge: Option<f32>,
    pub edge_width: Option<Pixels>,
    pub edge_aa: Option<Pixels>,
    /// Replaces the preset's tint color.
    pub tint: Option<Hsla>,
    /// Sets the tint alpha. Presets without a tint get their semantic color.
    pub tint_opacity: Option<f32>,
}

impl GlassSpec {
    fn theme(mode: Mode) -> Palette {
        let default = DefaultTheme::default();
        match mode {
            Mode::Light => default.light,
            Mode::Dark => default.dark,
        }
    }

    /// Glass for the default action, with a visible semantic tint.
    pub fn accent(mode: Mode, overrides: Option<GlassOverrides>) -> Self {
        let scheme: Palette = Self::theme(mode);
        let tint = scheme.accent.alpha(0.580);
        let base = GlassSpec {
            blur_radius: px(1.8),
            lens: px(1.0),
            magnify: 10.00,
            dispersion: 0.000,
            gain_light: 0.149,
            gain_dark: 0.149,
            saturation: 1.38,
            tint: Some(tint),
            edge: 0.24,
            edge_width: px(4.7),
            edge_aa: px(0.5),
            ..GlassSpec::default()
        };
        Self::render(base, scheme.accent, overrides)
    }

    /// Glass for a secondary action, with a quieter semantic tint.
    pub fn secondary(mode: Mode, overrides: Option<GlassOverrides>) -> Self {
        let scheme: Palette = Self::theme(mode);
        let base = GlassSpec {
            blur_radius: px(0.9),
            lens: px(4.0),
            magnify: 16.00,
            dispersion: 0.000,
            gain_light: 0.149,
            gain_dark: 0.149,
            saturation: 1.38,
            edge: 0.27,
            edge_width: px(0.4),
            edge_aa: px(0.5),
            ..GlassSpec::default()
        };
        Self::render(base, scheme.accent, overrides)
    }

    /// Glass for a destructive action, with a visible danger tint and rim.
    pub fn destructive(mode: Mode, overrides: Option<GlassOverrides>) -> Self {
        let scheme: Palette = Self::theme(mode);

        let base = GlassSpec {
            blur_radius: px(0.5),
            lens: px(2.0),
            magnify: -5.72,
            dispersion: 0.001,
            gain_light: 0.000,
            gain_dark: 0.000,
            saturation: 1.60,
            edge: 0.16,
            tint: Some(scheme.danger.alpha(0.580)),
            edge_width: px(6.3),
            edge_aa: px(0.5),
            ..GlassSpec::default()
        };
        Self::render(base, scheme.danger, overrides)
    }

    /// Applies any provided overrides on top of a preset's `base` spec.
    /// `semantic` is the preset's role color, used when `tint_opacity` is set
    /// on a preset that has no tint of its own.
    fn render(base: GlassSpec, semantic: Hsla, overrides: Option<GlassOverrides>) -> Self {
        let Some(o) = overrides else {
            return base;
        };
        let tint = match (o.tint, o.tint_opacity) {
            (Some(color), Some(alpha)) => Some(color.alpha(alpha)),
            (Some(color), None) => Some(color),
            (None, Some(alpha)) => Some(base.tint.unwrap_or(semantic).alpha(alpha)),
            (None, None) => base.tint,
        };
        Self {
            blur_radius: o.blur_radius.unwrap_or(base.blur_radius),
            lens: o.lens.unwrap_or(base.lens),
            reach: o.reach.unwrap_or(base.reach),
            magnify: o.magnify.unwrap_or(base.magnify),
            dispersion: o.dispersion.unwrap_or(base.dispersion),
            gain_light: o.gain_light.unwrap_or(base.gain_light),
            gain_dark: o.gain_dark.unwrap_or(base.gain_dark),
            saturation: o.saturation.unwrap_or(base.saturation),
            tint,
            edge: o.edge.unwrap_or(base.edge),
            edge_width: o.edge_width.unwrap_or(base.edge_width),
            edge_aa: o.edge_aa.unwrap_or(base.edge_aa),
        }
    }
}

/// Adds the app's glass material behind a styled element and its subtree.
pub trait GlassSurfaceExt: Styled + IntoElement + Sized {
    /// Uses the element's corner radii and the active theme's neutral glass tint.
    fn glass(mut self) -> GlassSurface {
        let corners = corners_of(&mut self);
        self.style().background = Some(gpui::transparent_black().into());
        GlassSurface::new(corners, GlassSpec::default(), self)
    }

    /// Uses the element's corner radii and a caller-supplied semantic tint.
    fn tinted_glass(mut self, tint: Hsla) -> GlassSurface {
        let corners = corners_of(&mut self);
        self.style().background = Some(gpui::transparent_black().into());
        GlassSurface::new(
            corners,
            GlassSpec {
                tint: Some(tint),
                ..GlassSpec::default()
            },
            self,
        )
    }

    /// Uses the element's corner radii and caller-supplied material parameters.
    fn glass_with(mut self, spec: GlassSpec) -> GlassSurface {
        let corners = corners_of(&mut self);
        self.style().background = Some(gpui::transparent_black().into());
        GlassSurface::new(corners, spec, self)
    }
}

impl<E: Styled + IntoElement> GlassSurfaceExt for E {}

/// A value-like wrapper that paints glass beneath one element subtree.
pub struct GlassSurface {
    corners: Corners<AbsoluteLength>,
    spec: GlassSpec,
    child: AnyElement,
}

impl GlassSurface {
    fn new(corners: Corners<AbsoluteLength>, spec: GlassSpec, child: impl IntoElement) -> Self {
        Self {
            corners,
            spec,
            child: child.into_any_element(),
        }
    }

    fn resolved_corners(&self, bounds: Bounds<Pixels>, rem: Pixels) -> Corners<Pixels> {
        Corners {
            top_left: self.corners.top_left.to_pixels(rem),
            top_right: self.corners.top_right.to_pixels(rem),
            bottom_right: self.corners.bottom_right.to_pixels(rem),
            bottom_left: self.corners.bottom_left.to_pixels(rem),
        }
        .clamp_radii_for_quad_size(bounds.size)
    }
}

impl Element for GlassSurface {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        self.child.id()
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        self.child.source_location()
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let corners = self.resolved_corners(bounds, window.rem_size());
        window.paint_layer(bounds, |window| {
            glass::paint(bounds, corners, self.spec, window, cx);
            self.child.paint(window, cx);
        });
    }
}

impl IntoElement for GlassSurface {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

fn corners_of(styled: &mut impl Styled) -> Corners<AbsoluteLength> {
    let square = AbsoluteLength::from(px(0.));
    let radii = &styled.style().corner_radii;
    Corners {
        top_left: radii.top_left.unwrap_or(square),
        top_right: radii.top_right.unwrap_or(square),
        bottom_right: radii.bottom_right.unwrap_or(square),
        bottom_left: radii.bottom_left.unwrap_or(square),
    }
}

#[cfg(test)]
#[path = "material.test.rs"]
mod tests;
