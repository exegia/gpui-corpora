#[allow(unused_imports)]
use super::*;

pub(crate) fn default_show_frame_time() -> bool {
    cfg!(debug_assertions)
}
