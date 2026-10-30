#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationInboxItem {
    pub id: NotificationId,
    pub created_at: AppTimestamp,
    pub title: String,
    pub body: String,
    pub read: bool,
    pub backend: String,
    pub delivered_natively: bool,
    pub degraded: bool,
    pub error_summary: Option<String>,
    pub kind: NotificationInboxKind,
}

impl NotificationInboxItem {
    pub fn summary_line(&self) -> String {
        let read_state = crate::i18n::localize(if self.read {
            "notifications_status_read"
        } else {
            "notifications_status_unread"
        });
        format!("{} • {read_state}", self.backend)
    }
}
