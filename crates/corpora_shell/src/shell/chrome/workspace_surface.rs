use gpui::{BoxShadow, StyleRefinement, hsla, prelude::*, px, rgb};

pub(super) fn style(dark: bool) -> StyleRefinement {
    let (fill, lit_blur, lit_alpha, dim_offset, dim_alpha) = if dark {
        (0x181818, 1.0, 0.17, -2.0, 0.1)
    } else {
        (0xe9eaeaeb, 2.0, 0.82, -3.0, 0.07)
    };
    StyleRefinement::default()
        .border_0()
        .p_0()
        .bg(rgb(fill))
        // GPUI paints shadows in order; CSS lists the topmost shadow first.
        .shadow(vec![
            BoxShadow::new(px(0.0), px(dim_offset), hsla(0.0, 0.0, 0.0, dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            BoxShadow::new(px(0.0), px(2.0), hsla(0.0, 0.0, 1.0, lit_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
        ])
}
