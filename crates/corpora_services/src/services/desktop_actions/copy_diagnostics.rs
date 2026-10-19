#[allow(unused_imports)]
use super::*;

pub fn copy_diagnostics(cx: &mut App) -> Result<(), DesktopActionError> {
    let diagnostics = build_diagnostics_text(cx);
    copy_text(&diagnostics, cx)
}
