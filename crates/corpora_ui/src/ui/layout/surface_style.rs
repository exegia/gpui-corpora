use crate::ely::theme::Mode;
use crate::styles::palette::color;
use gpui::{BoxShadow, Hsla, StyleRefinement, hsla, prelude::*, px};

pub(super) fn style(dark: bool, flush_right: bool) -> StyleRefinement {
    let mode = if dark { Mode::Dark } else { Mode::Light };
    let (lit_blur, lit_alpha, dim_offset, dim_alpha) = if dark {
        (0.7, 0.18, -2.5, 0.5)
    } else {
        (1.0, 1.0, -2.0, 0.08)
    };
    let fill = Hsla {
        a: 1.0,
        ..color("surface", mode)
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
        .bg(fill)
        // GPUI paints shadows in order; CSS lists the topmost shadow first.
        .shadow(vec![
            // The dim shadow is painted first, so it appears behind the lit shadow.
            BoxShadow::new(px(0.0), px(dim_offset), hsla(0.0, 0.0, 0.0, dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // The lit shadow is painted on top of the dim shadow, so it appears brighter.
            BoxShadow::new(px(0.7), px(2.), hsla(0.0, 0.0, 1.0, lit_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
            // Remove the shadow on the right side
            BoxShadow::new(px(-1.5), px(0.0), hsla(0.0, 0.0, 0.0, dim_alpha))
                .blur_radius(px(lit_blur))
                .inset(),
        ])
}
