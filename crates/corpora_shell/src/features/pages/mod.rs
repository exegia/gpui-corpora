mod about;
mod diagnostics;
mod notifications;
mod render_error;
mod settings;

pub use about::AboutPage;
pub use diagnostics::DiagnosticsPage;
pub use notifications::NotificationsPage;
pub use render_error::{ReloadCurrentPage, RenderErrorPage, TriggerRenderError};
pub use settings::SettingsPage;
