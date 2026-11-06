#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct DesktopActionsSnapshot {
    pub clipboard_available: bool,
    pub picker_available: bool,
    pub opener_available: bool,
    pub active_watchers: usize,
    pub last_error: Option<String>,
}

impl Default for DesktopActionsSnapshot {
    fn default() -> Self {
        Self {
            clipboard_available: false,
            picker_available: true,
            opener_available: true,
            active_watchers: 0,
            last_error: None,
        }
    }
}
