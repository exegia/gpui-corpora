#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn pending_swap_path_is_consistent() {
    let a = pending_swap_path();
    let b = pending_swap_path();
    assert_eq!(
        a, b,
        "pending_swap_path should return the same path on every call"
    );
}
