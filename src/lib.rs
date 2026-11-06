// Must stay at the crate root: rust_i18n::t! resolves crate::_rust_i18n_t!
// relative to this module.
rust_i18n::i18n!("locales", fallback = "en");

pub mod app;
pub mod entry;
pub mod platform;
pub mod services;

pub use corpora_config::state;
pub use corpora_foundation::{foundation, navigation};
pub use corpora_persistence::persistence;
pub use corpora_platform::runtime;
pub use corpora_shell::{features, pages, routes, shell};
pub use corpora_ui::ui;

pub use app::lifecycle;
pub use features::pages as views;
pub use foundation::{errors, ids, time};
pub use persistence::sqlite::db_migrations;
pub use platform::filesystem::paths;
pub use platform::input::shortcuts;
pub use platform::ipc;
pub use platform::process::single_instance;
pub use runtime::{capabilities, events};
pub use services::{
    accessibility, commands, connectivity, crash_report, desktop_actions, error_surface, first_run,
    i18n, logging, notifications, secure_storage, session, storage, tasks, telemetry, undo_stack,
    updater,
};
pub use state::config_store as app_state;
pub use state::migrations as config_migrations;
