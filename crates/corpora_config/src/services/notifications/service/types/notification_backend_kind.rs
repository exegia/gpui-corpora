#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationBackendKind {
    UserNotify,
    NotifyRust,
    #[cfg(feature = "notifications-portal")]
    Portal,
    UiOnly,
}

impl fmt::Display for NotificationBackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserNotify => f.write_str("user-notify"),
            Self::NotifyRust => f.write_str("notify-rust"),
            #[cfg(feature = "notifications-portal")]
            Self::Portal => f.write_str("xdg-portal"),
            Self::UiOnly => f.write_str("in-app only"),
        }
    }
}
