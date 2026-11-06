#[allow(unused_imports)]
use super::*;

pub(crate) fn apply_inverse(kind: &UndoKind, cx: &mut App) {
    match kind {
        UndoKind::ThemeMode { before, .. } => {
            crate::app::set_theme_mode_with_record(*before, false, cx)
        }
    }
}
