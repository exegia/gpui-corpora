#[allow(unused_imports)]
use super::*;

// Shell APIs persist long paths while TEMP/current_exe can arrive as 8.3
// short forms (RUNNER~1), so a raw text compare never matches. Also
// case-insensitive; deep_link_registration reuses this for its command value.
pub(crate) fn same_path(a: &Path, b: &Path) -> bool {
    comparable(a) == comparable(b)
}
