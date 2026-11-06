#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn parses_supported_deep_links() {
    assert_eq!(parse_deep_link("corpora://home").unwrap(), "/");
    assert_eq!(parse_deep_link("corpora://ui").unwrap(), "/ui");
    assert_eq!(
        parse_deep_link("corpora://ui/breadcrumb").unwrap(),
        "/ui/breadcrumb"
    );
    assert_eq!(
        parse_deep_link("corpora://settings/notifications").unwrap(),
        "/settings/notifications"
    );
    assert_eq!(
        parse_deep_link("corpora://diagnostics").unwrap(),
        "/diagnostics"
    );
    assert_eq!(
        parse_deep_link("corpora://explore/detail/text/42/viewer").unwrap(),
        "/explore/detail/text/42/viewer"
    );
}
