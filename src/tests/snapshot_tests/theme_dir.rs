#[allow(unused_imports)]
use super::*;

pub(crate) fn theme_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("themes")
}
