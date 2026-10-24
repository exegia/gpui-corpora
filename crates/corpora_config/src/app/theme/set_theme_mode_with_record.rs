#[allow(unused_imports)]
use super::*;

/// Cross-fade to `mode` and persist it; `record` adds an undo entry.
pub fn set_theme_mode_with_record(mode: ThemeMode, record: bool, cx: &mut App) {
    let before = current_mode(cx);
    Theme::set_mode(mode.into(), cx);
    crate::app_state::update_config(cx, |config| config.theme = mode);
    if record && before != mode {
        crate::undo_stack::record_theme_mode_change(before, mode, cx);
    }
}
