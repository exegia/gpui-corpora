#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn window_icon_is_the_48px_brand_png() {
    let icon = window_icon().expect("48px brand png in the embedded set");
    assert_eq!(icon.dimensions(), (48, 48));
    assert!(icon.pixels().any(|px| px.0[3] > 0));
}
