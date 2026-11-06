#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting { attempt: u8 },
    Closed,
}
