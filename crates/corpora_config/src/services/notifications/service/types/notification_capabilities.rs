#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NotificationCapabilities {
    pub can_request_permission: bool,
    pub can_read_permission_state: bool,
    pub can_send_immediate_native: bool,
    pub can_send_interactive: bool,
    pub requires_packaged_runtime: bool,
}
