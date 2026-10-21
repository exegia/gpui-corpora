#[allow(unused_imports)]
use super::*;

/// Retained pages owned by one workspace tab.
#[derive(Clone)]
pub struct Pages {
    pub settings: Entity<SettingsPage>,
    pub notifications: Entity<NotificationsPage>,
    pub diagnostics: Entity<DiagnosticsPage>,
    pub about: Entity<AboutPage>,
}
