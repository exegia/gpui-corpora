#[allow(unused_imports)]
use super::*;

/// Lowercased long form; paths that cannot be expanded (not on disk) fall
/// back to the raw lowercased text.
pub(crate) fn comparable(path: &Path) -> String {
    let raw = path.to_string_lossy();
    match long_form(&raw) {
        Some(long) => long.to_lowercase(),
        None => raw.to_lowercase(),
    }
}
