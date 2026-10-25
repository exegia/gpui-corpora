use gpui::Hsla;

use crate::ely::theme::Palette;

pub(super) const VERTICAL_TAB_RADIUS: f32 = 14.;
pub(super) const VERTICAL_TAB_LABEL_SIZE: f32 = 9.;

/// Workspace tab colors are shared by both layouts.
pub(super) fn content_color(colors: &Palette, active: bool) -> Hsla {
    if active {
        colors.fg
    } else {
        colors.fg.opacity(0.6)
    }
}
