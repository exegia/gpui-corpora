use crate::{ely::theme::Palette, styles::utils::ColorExt};
use gpui::{BoxShadow, Hsla, StyleRefinement, prelude::*, px};

pub(super) fn style(colors: &Palette, dark: bool, flush_right: bool) -> StyleRefinement {
    let (lit_blur, lit_alpha, dim_offset, dim_alpha) = if dark {
        (0.7, 0.18, -2.5, 0.5)
    } else {
        (1.0, 1.0, -2.0, 0.08)
    };
    let fill = Hsla {
        a: 1.0,
        ..colors.surface.darken(if dark { 0.2 } else { 0.0 })
    };
    let style = StyleRefinement::default();
    let style = if flush_right {
        style
            .rounded_tr_none()
            .rounded_br_none()
            .rounded_tl_xl()
            .rounded_bl_xl()
    } else {
        style.rounded_xl()
    };
    style
        .border_0()
        .p_0()
        .bg(fill)
        // GPUI paints shadows in order; CSS lists the topmost shadow first.
        .shadow(vec![
            // The dim shadow is painted first, so it appears behind the lit shadow.
            BoxShadow::new(px(0.0), px(dim_offset), colors.shadow.opacity(dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // The lit shadow is painted on top of the dim shadow, so it appears brighter.
            BoxShadow::new(px(0.7), px(2.), colors.on_media.opacity(lit_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // Remove the shadow on the right side
            BoxShadow::new(px(-1.5), px(0.0), colors.shadow.opacity(dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
        ])
}
