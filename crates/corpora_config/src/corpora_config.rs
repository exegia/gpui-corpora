//! Persisted app config and the state it owns: theme, locale, notifications,
//! and the undo stack.

// Must stay at the crate root: rust_i18n::t! resolves crate::_rust_i18n_t!
// relative to this module.
rust_i18n::i18n!("../../locales", fallback = "en");

#[allow(unused_imports)]
pub(crate) use corpora_foundation::{errors, foundation, ids, navigation, time};
#[allow(unused_imports)]
pub(crate) use corpora_platform::{capabilities, events, ipc, paths, runtime, single_instance};
#[allow(unused_imports)]
pub(crate) use corpora_runtime_services::{
    accessibility, connectivity, i18n, secure_storage, session, streaming, tasks, telemetry,
    tokio_runtime,
};
#[allow(unused_imports)]
pub(crate) use corpora_ui::ui;

pub mod state;

pub mod app {
    pub mod locale;
    pub mod theme;

    pub use locale::{LOCALE_EN, LOCALE_ZH_CN, LocaleState, current_locale, set_locale};
    pub use theme::{set_theme_mode, set_theme_mode_with_record};
}

pub mod platform {
    #[allow(unused_imports)]
    pub(crate) use corpora_platform::platform::*;

    #[cfg(target_os = "windows")]
    pub mod win_toast;
}

pub mod services {
    #[allow(unused_imports)]
    pub(crate) use corpora_runtime_services::services::*;

    pub mod notifications;
    pub mod undo_stack;
}

pub use services::{notifications, undo_stack};
pub use state::config_store as app_state;
pub use state::migrations as config_migrations;
