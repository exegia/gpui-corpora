#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationImportance {
    ForegroundOnly,
    BackgroundWorthy,
}

impl fmt::Display for NotificationImportance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForegroundOnly => f.write_str("foreground-only"),
            Self::BackgroundWorthy => f.write_str("background-worthy"),
        }
    }
}
