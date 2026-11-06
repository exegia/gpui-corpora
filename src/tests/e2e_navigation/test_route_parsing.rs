#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_route_parsing() {
    // --- Top-level page routes -------------------------------------------
    let cases = &[
        ("gpui-starter://home", AppRoute::Page(Page::Home)),
        ("gpui-starter://form", AppRoute::Page(Page::Form)),
        ("gpui-starter://settings", AppRoute::Page(Page::Settings)),
        (
            "gpui-starter://notifications",
            AppRoute::Page(Page::Notifications),
        ),
        (
            "gpui-starter://diagnostics",
            AppRoute::Page(Page::Diagnostics),
        ),
        ("gpui-starter://about", AppRoute::Page(Page::About)),
        // Sub-route
        (
            "gpui-starter://settings/notifications",
            AppRoute::SettingsNotifications,
        ),
    ];

    for (url, expected) in cases {
        let parsed =
            AppRoute::parse_deep_link(url).unwrap_or_else(|e| panic!("failed to parse {url}: {e}"));
        assert_eq!(
            parsed, *expected,
            "parse_deep_link({url}): expected {expected:?}, got {parsed:?}"
        );
    }

    // --- Round-trip: parse → to_url → parse ------------------------------
    for (url, _expected) in cases {
        let route = AppRoute::parse_deep_link(url).unwrap();
        let serialized = route.to_url();
        let reparsed = AppRoute::parse_deep_link(&serialized).unwrap();
        assert_eq!(
            reparsed, route,
            "round-trip failed for {url}: {route:?} → {serialized} → {reparsed:?}"
        );
    }

    // --- Invalid inputs --------------------------------------------------
    let bad = &[
        "https://example.com",
        "gpui-starter://missing-page",
        "gpui-starter://settings/..%2Fetc",
        "",
    ];

    for url in bad {
        assert!(
            AppRoute::parse_deep_link(url).is_err(),
            "expected parse failure for {url:?}"
        );
    }
}
