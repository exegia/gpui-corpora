#[allow(unused_imports)]
use super::*;

/// Retained pages owned by one workspace tab.
#[derive(Clone)]
pub struct Pages {
    #[cfg(debug_assertions)]
    pub ui: Entity<crate::routes::ui::Ui>,
    #[cfg(debug_assertions)]
    pub button: Entity<crate::routes::ui::button::ButtonShowcase>,
    pub notifications: Entity<NotificationsPage>,
    pub diagnostics: Entity<DiagnosticsPage>,
    pub about: Entity<AboutPage>,
}
