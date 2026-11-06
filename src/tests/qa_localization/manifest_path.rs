#[allow(unused_imports)]
use super::*;

pub(crate) fn manifest_path(relative: &str) -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(relative)
}
