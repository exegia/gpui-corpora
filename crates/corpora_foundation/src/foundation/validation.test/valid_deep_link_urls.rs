#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn valid_deep_link_urls() {
    assert!(validate_deep_link_url("corpora://home").is_ok());
    assert!(validate_deep_link_url("corpora://settings").is_ok());
    // "http" was an orphan host — its feature (http_lab) was removed in the
    // gpui-query migration, so it is correctly rejected now.
    assert!(validate_deep_link_url("corpora://http").is_err());
    assert!(validate_deep_link_url("corpora://settings/notifications").is_ok());
    assert!(validate_deep_link_url("corpora://diagnostics").is_ok());
    assert!(validate_deep_link_url("corpora://notifications").is_ok());
    assert!(validate_deep_link_url("corpora://about").is_ok());
    assert!(validate_deep_link_url("corpora://projects").is_ok());
    assert!(validate_deep_link_url("corpora://form").is_err());
}
