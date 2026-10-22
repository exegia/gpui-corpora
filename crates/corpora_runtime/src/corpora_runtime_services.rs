//! Leaf services: i18n, tokio runtime, session, accessibility, connectivity,
//! secure storage, streaming, telemetry, and background tasks.

// Must stay at the crate root: rust_i18n::t! resolves crate::_rust_i18n_t!
// relative to this module.
rust_i18n::i18n!("../../locales", fallback = "en");

#[allow(unused_imports)]
pub(crate) use corpora_foundation::{errors, foundation, ids, navigation, time};
#[allow(unused_imports)]
pub(crate) use corpora_platform::{
    capabilities, events, ipc, paths, platform, runtime, single_instance,
};
#[allow(unused_imports)]
pub(crate) use corpora_ui::ui;

pub mod services {
    pub mod accessibility;
    pub mod connectivity;
    pub mod i18n;
    pub mod secure_storage;
    pub mod session;
    pub mod streaming;
    pub mod tasks;
    pub mod telemetry;
    pub mod tokio_runtime;
}

pub use services::{
    accessibility, connectivity, i18n, secure_storage, session, streaming, tasks, telemetry,
    tokio_runtime,
};
