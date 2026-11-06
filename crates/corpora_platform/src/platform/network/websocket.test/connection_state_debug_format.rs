#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn connection_state_debug_format() {
    let state = ConnectionState::Reconnecting { attempt: 3 };
    let s = format!("{:?}", state);
    assert!(s.contains("attempt"));
}
