//! Glass surface wrapper adapted from Bezel's MIT-licensed `ui::surface` module.

use gpui::{
    AbsoluteLength, AnyElement, App, Bounds, Corners, Element, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, Pixels, Styled, Window, px,
};

use super::glass;

pub use super::glass::GlassSpec;

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
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
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
