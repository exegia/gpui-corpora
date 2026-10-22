use gpui::{BoxShadow, StyleRefinement, hsla, prelude::*, px, rgb};

pub(super) fn style(dark: bool, flush_right: bool) -> StyleRefinement {
    let (fill, lit_blur, lit_alpha, dim_offset, dim_alpha) = if dark {
        (0x181818, 0.7, 0.18, -2.5, 0.5)
    } else {
        (0xe9eaeaeb, 1.0, 0.95, -2.0, 0.08)
    };
    let style = StyleRefinement::default();
    let style = if flush_right {
        style.rounded_tr(px(0.0)).rounded_br(px(0.0))
    } else {
        style
    };
    style
        .border_0()
        .p_0()
        .bg(rgb(fill))
        // GPUI paints shadows in order; CSS lists the topmost shadow first.
        .shadow(vec![
            // The dim shadow is painted first, so it appears behind the lit shadow.
            BoxShadow::new(px(0.0), px(dim_offset), hsla(0.0, 0.0, 0.0, dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // The lit shadow is painted on top of the dim shadow, so it appears brighter.
            BoxShadow::new(px(0.7), px(1.5), hsla(0.0, 0.0, 1.0, lit_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // Remove the shadow on the right side
            BoxShadow::new(px(-1.5), px(0.0), hsla(0.0, 0.0, 0.0, dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
        ])
}
