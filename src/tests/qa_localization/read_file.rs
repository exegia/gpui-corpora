#[allow(unused_imports)]
use super::*;

pub(crate) fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}
