//! OS integration: paths, IPC, single instance, networking, runtime
//! capabilities and events, and process reload.

#[allow(unused_imports)]
pub(crate) use corpora_foundation::{errors, foundation, ids, navigation, time};

pub mod runtime;

pub mod app {
    pub mod reload;

    pub use reload::{is_reload_requested, perform_reload, request_reload};
}

pub mod platform {
    #[cfg(target_os = "windows")]
    pub mod deep_link_registration;
    pub mod environment;
    pub mod filesystem;
    pub mod ipc;
    pub mod network;
    pub mod process;
    #[cfg(target_os = "windows")]
    pub mod toast_identity;
}

pub use platform::filesystem::paths;
pub use platform::ipc;
pub use platform::process::single_instance;
pub use runtime::{capabilities, events};
