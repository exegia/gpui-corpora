//! Route paths and `corpora://` deep links for the gpui-router tree in `crate::routes`.

pub mod deep_link;

pub use deep_link::{APP_URL_SCHEME, parse_deep_link, to_url};

mod about;
mod diagnostics;
mod explore;
mod home;
mod notifications;
mod profile;
mod projects;
mod sections;
mod settings;
mod settings_notifications;

pub use about::ABOUT;
pub use diagnostics::DIAGNOSTICS;
pub use explore::EXPLORE;
pub use home::HOME;
pub use notifications::NOTIFICATIONS;
pub use profile::PROFILE;
pub use projects::PROJECTS;
pub use sections::SECTIONS;
pub use settings::SETTINGS;
pub use settings_notifications::SETTINGS_NOTIFICATIONS;
