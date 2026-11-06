//! App services built on the config layer: storage, crash reports, error
//! surfacing, desktop actions, logging, first run, updater, and lifecycle.

#[allow(unused_imports)]
pub(crate) use corpora_config::{app_state, config_migrations, notifications, state, undo_stack};
#[allow(unused_imports)]
pub(crate) use corpora_foundation::{errors, foundation, ids, navigation, time};
#[allow(unused_imports)]
pub(crate) use corpora_persistence::{db_migrations, persistence};
#[allow(unused_imports)]
pub(crate) use corpora_platform::{
    capabilities, events, ipc, paths, platform, runtime, single_instance,
};
#[allow(unused_imports)]
pub(crate) use corpora_runtime::{
    accessibility, connectivity, i18n, secure_storage, session, streaming, tasks, telemetry,
    tokio_runtime,
};

pub mod services;

pub mod app {
    #[allow(unused_imports)]
    pub(crate) use corpora_config::app::*;

    pub mod lifecycle;
}

pub use app::lifecycle;
pub use services::{
    crash_report, desktop_actions, error_surface, first_run, logging, storage, updater,
};
