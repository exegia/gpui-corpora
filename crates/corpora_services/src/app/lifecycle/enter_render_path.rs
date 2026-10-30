#[allow(unused_imports)]
use super::*;

/// Only render-originating panics trigger the error boundary.
pub fn enter_render_path() -> impl Drop {
    IN_RENDER_PATH.with(|c| c.set(true));
    RenderPathGuard
}
