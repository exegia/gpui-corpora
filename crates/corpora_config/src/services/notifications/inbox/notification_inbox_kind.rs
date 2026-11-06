#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationInboxKind {
    Attempt,
    PermissionUpdate,
    SettingsUpdate,
}
