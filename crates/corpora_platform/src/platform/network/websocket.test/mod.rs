use super::*;

#[cfg(feature = "websocket")]
mod ws_url_allowlist {
    use super::client::validate_ws_url;

    #[test]
    fn accepts_only_ws_and_wss_urls() {
        for url in ["ws://example.com/chat", "wss://example.com/chat"] {
            assert!(validate_ws_url(url).is_ok(), "{url} should be dialable");
        }
    }

    #[test]
    fn rejects_non_ws_schemes_and_schemeless_urls() {
        for url in [
            "http://example.com",
            "https://example.com",
            "file:///etc/passwd",
            "gopher://example.com",
            "WSS://example.com", // scheme match is exact, case-sensitive
            "example.com",
            "ws:",
            "wss:",
        ] {
            assert!(validate_ws_url(url).is_err(), "{url} must be refused");
        }
    }
}

// === split-modules: generated submodules ===
mod connection_state_debug_format;
mod reconnect_delay_increases_exponentially;
mod reconnect_delay_respects_cap;
mod reconnect_policy_default_limits_retries;
mod stub_client_returns_error_without_feature;
