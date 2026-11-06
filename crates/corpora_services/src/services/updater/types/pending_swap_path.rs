#[allow(unused_imports)]
use super::*;

pub(crate) fn pending_swap_path() -> Option<PathBuf> {
    Some(updates_dir()?.join("pending-swap.json"))
}
