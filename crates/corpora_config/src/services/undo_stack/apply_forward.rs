#[allow(unused_imports)]
use super::*;

pub(crate) fn apply_forward(kind: &UndoKind, cx: &mut App) {
    match kind {
        UndoKind::ThemeMode { after, .. } => {
            crate::app::set_theme_mode_with_record(*after, false, cx)
        }
    }
}
