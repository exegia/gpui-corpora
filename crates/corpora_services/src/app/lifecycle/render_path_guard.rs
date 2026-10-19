#[allow(unused_imports)]
use super::*;

pub(crate) struct RenderPathGuard;

impl Drop for RenderPathGuard {
    fn drop(&mut self) {
        IN_RENDER_PATH.with(|c| c.set(false));
    }
}
