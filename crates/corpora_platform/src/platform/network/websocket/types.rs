//! Types for the WebSocket client scaffold, always compiled.

mod connection_state;
mod message_handler;
mod reconnect_policy;
mod web_socket_error;

pub use connection_state::ConnectionState;
pub use message_handler::MessageHandler;
pub use reconnect_policy::ReconnectPolicy;
pub use web_socket_error::WebSocketError;
