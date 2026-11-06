#[allow(unused_imports)]
use super::*;

pub fn in_render_path() -> bool {
    IN_RENDER_PATH.with(|c| c.get())
}
