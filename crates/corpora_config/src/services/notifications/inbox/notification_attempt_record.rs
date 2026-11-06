#[allow(unused_imports)]
use super::*;

pub struct NotificationAttemptRecord {
    pub title: String,
    pub body: String,
    pub backend: NotificationBackendKind,
    pub delivered_natively: bool,
    pub degraded: bool,
    pub error_summary: Option<String>,
    pub kind: NotificationInboxKind,
}
