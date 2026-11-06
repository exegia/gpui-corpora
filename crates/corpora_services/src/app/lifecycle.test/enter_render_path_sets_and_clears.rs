#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn enter_render_path_sets_and_clears() {
    assert!(!in_render_path());
    {
        let _guard = enter_render_path();
        assert!(in_render_path());
    }
    assert!(!in_render_path());
}
