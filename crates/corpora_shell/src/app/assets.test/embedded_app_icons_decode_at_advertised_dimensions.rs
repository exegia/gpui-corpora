#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn embedded_app_icons_decode_at_advertised_dimensions() {
    for size in [16u16, 32, 48] {
        let png = app_icon_png(size).expect("size is in the committed hicolor set");
        assert_eq!(png.dimensions(), (u32::from(size), u32::from(size)));
        assert!(png.pixels().any(|px| px.0[3] > 0));
    }
}
