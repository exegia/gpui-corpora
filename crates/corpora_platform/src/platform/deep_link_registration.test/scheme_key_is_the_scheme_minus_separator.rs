#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn scheme_key_is_the_scheme_minus_separator() {
    assert_eq!(scheme_key(), "corpora");
}
