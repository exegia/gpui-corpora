#[allow(unused_imports)]
use super::*;

/// Pages that hold subscriptions live as entities for the window's life;
/// stateless Corpora pages render from `pages` on demand.
pub struct Pages {
    pub settings: Entity<SettingsPage>,
    pub notifications: Entity<NotificationsPage>,
    pub diagnostics: Entity<DiagnosticsPage>,
    pub about: Entity<AboutPage>,
}
