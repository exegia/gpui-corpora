use gpui::{BoxShadow, Hsla, StyleRefinement, prelude::*, px, rgb};

struct InsetShadow {
    alpha: f32,
    base: u32,
    blur: f32,
}

fn hex_to_hsla(hex: u32, alpha: f32) -> Hsla {
    let mut color: Hsla = rgb(hex).into();
    color.a = alpha;
    color
}

/// A dim and a lit inset shadow, `offset` pixels down, for refining onto anything.
pub fn bg_inset(dark: bool, offset: f32) -> StyleRefinement {
    let mut dim = InsetShadow {
        alpha: 0.08,
        base: 0x000000,
        blur: 1.0,
    };
    let lit = InsetShadow {
        alpha: 1.0,
        base: 0xffffff,
        blur: 0.0,
    };

    if dark {
        dim.base = 0x181818;
        // lit.base = 0x2e2e2e;
    }
    StyleRefinement::default()
        // GPUI paints shadows in order; CSS lists the topmost shadow first.
        .shadow(vec![
            // The dim shadow is painted first, so it appears behind the lit shadow.
            BoxShadow::new(px(0.0), px(offset), hex_to_hsla(dim.base, dim.alpha))
                .blur_radius(px(dim.blur))
                .inset(),
            // The lit shadow is painted on top of the dim shadow, so it appears brighter.
            BoxShadow::new(px(0.0), px(offset), hex_to_hsla(lit.base, lit.alpha))
                .blur_radius(px(lit.blur))
                .inset(),
        ])
}
