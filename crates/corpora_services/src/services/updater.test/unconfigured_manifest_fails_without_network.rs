use super::*;

#[test]
fn unconfigured_manifest_fails_without_network() {
    if DEFAULT_MANIFEST_URL.is_some() {
        return;
    }
    let runtime = std::sync::Arc::new(tokio::runtime::Runtime::new().unwrap());
    let result = runtime.block_on(super::super::check::fetch_manifest(
        runtime.clone(),
        reqwest::Client::new(),
    ));
    assert_eq!(
        result.unwrap_err(),
        "Updates are not configured for this build"
    );
}
