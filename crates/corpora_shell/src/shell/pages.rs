#[allow(unused_imports)]
use super::*;

/// Retained pages owned by one workspace tab.
#[derive(Clone)]
pub struct Pages {
    pub ui: Entity<crate::routes::ui::Ui>,
    pub button: Entity<crate::routes::ui::button::ButtonShowcase>,
    pub notifications: Entity<NotificationsPage>,
    pub diagnostics: Entity<DiagnosticsPage>,
    pub about: Entity<AboutPage>,
}
