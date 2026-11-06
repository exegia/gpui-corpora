#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotificationPermissionState {
    Unknown,
    Unsupported,
    Unavailable(String),
    NotDetermined,
    Denied,
    Authorized,
}

impl NotificationPermissionState {
    pub fn label(&self) -> SharedString {
        let key = match self {
            Self::Unknown => "notifications_permission_unknown",
            Self::Unsupported => "notifications_permission_unsupported",
            Self::Unavailable(_) => "notifications_permission_unavailable",
            Self::NotDetermined => "notifications_permission_not_requested",
            Self::Denied => "notifications_permission_denied",
            Self::Authorized => "notifications_permission_authorized",
        };
        let label = crate::i18n::localize(key);
        match self {
            Self::Unavailable(reason) => format!("{label}: {reason}").into(),
            _ => label.into(),
        }
    }
}
