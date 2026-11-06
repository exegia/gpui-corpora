#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_encoded_separators_and_controls() {
    assert!(parse_deep_link("corpora://projects/a%2Fb").is_err());
    assert!(parse_deep_link("corpora://projects/%0Aid").is_err());
    assert_eq!(
        parse_deep_link("corpora://projects/detail/my%20project").unwrap(),
        "/projects/detail/my project"
    );
}
