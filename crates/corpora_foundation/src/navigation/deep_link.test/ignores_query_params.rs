#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn ignores_query_params() {
    assert_eq!(
        parse_deep_link("corpora://home?ref=test&source=menu").unwrap(),
        "/"
    );
}
