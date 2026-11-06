#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct NotificationSendResult {
    pub backend_used: NotificationBackendKind,
    pub degraded: bool,
    /// True iff the backend's send returned `Ok` — "accepted by the daemon",
    /// not proof the user saw a banner (FreeDesktop suppresses silently).
    pub delivered_natively: bool,
    pub error_summary: Option<SharedString>,
    pub importance: NotificationImportance,
}
