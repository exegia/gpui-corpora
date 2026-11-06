#[allow(unused_imports)]
use super::*;

/// Localizes an app-owned message key via this crate's rust-i18n backend
/// (initialized in `src/lib.rs`); a missing key renders as the key itself.
pub fn localize(id: &str) -> String {
    rust_i18n::t!(id).to_string()
}
