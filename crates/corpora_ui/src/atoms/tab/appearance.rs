use gpui::Hsla;

use crate::ely::theme::Palette;

/// Corner radius shared by a tab and its moving selection surface.
pub(super) const TAB_RADIUS: f32 = 14.;
/// Tab label text size.
pub(super) const TAB_LABEL_SIZE: f32 = 9.;

/// A tab label's color. The selected tab reads against the glass surface, which
/// differs by appearance; the rest use muted foreground.
pub(super) fn label_color(colors: &Palette, dark: bool, active: bool) -> Hsla {
    if active {
        if dark { colors.fg } else { colors.on_accent }
    } else {
        colors.fg_muted
    }
}
