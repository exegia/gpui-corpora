#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_route_parse_invalid() {
    let invalid_links = &[
        ("https://example.com", "wrong_scheme"),
        ("gpui-starter://unknown-host", "unknown_host"),
        ("gpui-starter://home/extra", "extra_segment"),
        ("not-a-url-at-all", "garbage"),
        ("gpui-starter://settings/..%2Fetc", "path_traversal"),
        ("", "empty_string"),
    ];

    let results: Vec<(&str, String)> = invalid_links
        .iter()
        .map(|(url, _label)| {
            let err = AppRoute::parse_deep_link(url).unwrap_err();
            (*url, err.to_string())
        })
        .collect();

    insta::assert_yaml_snapshot!(
            &results,
    @r#"
        - - "https://example.com"
          - "invalid deep link `https://example.com`: unsupported scheme `https`, expected `gpui-starter`"
        - - "gpui-starter://unknown-host"
          - "invalid deep link `gpui-starter://unknown-host`: unexpected host `unknown-host`"
        - - "gpui-starter://home/extra"
          - "invalid deep link `gpui-starter://home/extra`: unknown route"
        - - not-a-url-at-all
          - "invalid deep link `not-a-url-at-all`: relative URL without a base"
        - - "gpui-starter://settings/..%2Fetc"
          - "invalid deep link `..%2Fetc`: path traversal detected in URL segment"
        - - ""
          - "invalid deep link ``: relative URL without a base"
        "#
        );
}
