#[allow(unused_imports)]
use super::*;

pub(crate) fn platform_key() -> String {
    let os = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unknown"
    };
    let arch = std::env::consts::ARCH;
    format!("{os}-{arch}")
}
