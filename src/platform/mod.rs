#[cfg(target_os = "windows")]
pub use corpora_config::platform::win_toast;
#[cfg(target_os = "windows")]
pub use corpora_platform::platform::{deep_link_registration, toast_identity};
pub use corpora_platform::platform::{environment, filesystem, ipc, network, process};
pub use corpora_shell::platform::input;
