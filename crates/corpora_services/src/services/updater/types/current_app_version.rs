#[allow(unused_imports)]
use super::*;

pub(crate) fn current_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
