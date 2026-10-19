#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_route_parse_valid() {
    let valid_links = &[
        ("gpui-starter://home", "home"),
        ("gpui-starter://form", "form"),
        ("gpui-starter://settings", "settings"),
        ("gpui-starter://notifications", "notifications"),
        ("gpui-starter://diagnostics", "diagnostics"),
        ("gpui-starter://about", "about"),
        (
            "gpui-starter://settings/notifications",
            "settings_notifications",
        ),
        (
            "gpui-starter://home?ref=test&source=menu",
            "home_with_query",
        ),
    ];

    let results: Vec<(&str, String)> = valid_links
        .iter()
        .map(|(url, _label)| {
            let route = AppRoute::parse_deep_link(url)
                .unwrap_or_else(|e| panic!("expected `{url}` to parse successfully: {e}"));
            (*url, format!("{route:?}"))
        })
        .collect();

    insta::assert_yaml_snapshot!(
            &results,
    @r#"
        - - "gpui-starter://home"
          - Page(Home)
        - - "gpui-starter://form"
          - Page(Form)
        - - "gpui-starter://settings"
          - Page(Settings)
        - - "gpui-starter://notifications"
          - Page(Notifications)
        - - "gpui-starter://diagnostics"
          - Page(Diagnostics)
        - - "gpui-starter://about"
          - Page(About)
        - - "gpui-starter://settings/notifications"
          - SettingsNotifications
        - - "gpui-starter://home?ref=test&source=menu"
          - Page(Home)
        "#
        );
}
