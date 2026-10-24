#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn stub_client_returns_error_without_feature() {
    let mut client = WebSocketClient::new("wss://localhost".to_string());
    assert_eq!(client.state, ConnectionState::Disconnected);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async { client.connect_loop().await });
    assert!(result.is_err());
}
