#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn persisted_bounds_copies_pixel_geometry() {
    let bounds = Bounds {
        origin: point(px(24.0), px(-8.0)),
        size: size(px(1400.0), px(900.0)),
    };
    assert_eq!(
        persisted_bounds(bounds),
        PersistedWindowBounds {
            x: 24.0,
            y: -8.0,
            width: 1400.0,
            height: 900.0,
        }
    );
}
