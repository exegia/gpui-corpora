//! The app UI: workspace shell, routes, pages, feature views, window and
//! actions, keybindings, commands, and global shortcuts.

#[allow(unused_imports)]
pub(crate) use corpora_config::{app_state, config_migrations, notifications, state, undo_stack};
#[allow(unused_imports)]
pub(crate) use corpora_foundation::{errors, foundation, ids, navigation, time};
#[allow(unused_imports)]
pub(crate) use corpora_platform::{capabilities, events, ipc, paths, runtime, single_instance};
#[allow(unused_imports)]
pub(crate) use corpora_runtime::{
    accessibility, connectivity, i18n, secure_storage, session, streaming, tasks, telemetry,
    tokio_runtime,
};
#[allow(unused_imports)]
pub(crate) use corpora_services::{
    crash_report, desktop_actions, error_surface, first_run, lifecycle, logging, storage, updater,
};
#[allow(unused_imports)]
pub(crate) use corpora_ui::ui;

pub mod features;
pub mod pages;
pub mod routes;
pub mod shell;

pub mod app {
    #[allow(unused_imports)]
    pub(crate) use corpora_config::app::*;
    #[allow(unused_imports)]
    pub(crate) use corpora_platform::app::*;
    #[allow(unused_imports)]
    pub(crate) use corpora_services::app::*;

    pub mod actions;
    pub mod assets;
    pub mod keys;
    pub mod window;

    pub use actions::{
        About, ExecuteCommand, OpenDiagnostics, Quit, Restart, SelectLocale, SwitchThemeMode,
        ToggleSearch, TriggerTestPanic,
    };
    pub use window::open_workspace_window;
}

pub mod platform {
    #[cfg(target_os = "windows")]
    #[allow(unused_imports)]
    pub(crate) use corpora_config::platform::win_toast;
    #[allow(unused_imports)]
    pub(crate) use corpora_platform::platform::*;

    pub mod input;
}

pub mod services {
    #[allow(unused_imports)]
    pub(crate) use corpora_config::services::*;
    #[allow(unused_imports)]
    pub(crate) use corpora_runtime::services::*;
    #[allow(unused_imports)]
    pub(crate) use corpora_services::services::*;

    pub mod commands;
}

pub use features::pages as views;
pub use platform::input::shortcuts;
pub use services::commands;
