#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn persisted_bounds_keeps_fractional_dpi_values() {
    let bounds = Bounds {
        origin: point(px(10.5), px(0.0)),
        size: size(px(700.25), px(450.75)),
    };
    let persisted = persisted_bounds(bounds);
    assert_eq!((persisted.x, persisted.width), (10.5, 700.25));
    assert_eq!((persisted.y, persisted.height), (0.0, 450.75));
}
