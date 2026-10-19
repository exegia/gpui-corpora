#[allow(unused_imports)]
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("connection failed: {0}")]
    Connection(String),
    #[error("not a ws/wss URL: {0}")]
    InvalidUrl(String),
    #[cfg(feature = "websocket")]
    #[error("send failed: {0}")]
    Send(#[source] tokio_tungstenite::tungstenite::Error),
    #[cfg(feature = "websocket")]
    #[error("close failed: {0}")]
    Close(#[source] tokio_tungstenite::tungstenite::Error),
    #[error("not connected")]
    NotConnected,
    #[error("websocket feature not enabled")]
    FeatureDisabled,
}
