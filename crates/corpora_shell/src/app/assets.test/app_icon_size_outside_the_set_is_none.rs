#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn app_icon_size_outside_the_set_is_none() {
    assert!(app_icon_png(20).is_none());
}
