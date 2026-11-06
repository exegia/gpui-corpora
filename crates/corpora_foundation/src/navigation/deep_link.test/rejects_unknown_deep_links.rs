#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn rejects_unknown_deep_links() {
    assert!(parse_deep_link("https://example.com").is_err());
    assert!(parse_deep_link("corpora://missing").is_err());
    assert!(parse_deep_link("corpora://home/extra").is_err());
}
