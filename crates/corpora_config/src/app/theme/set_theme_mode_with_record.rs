#[allow(unused_imports)]
use super::*;

/// Cross-fade to `mode` and persist it; `record` adds an undo entry.
pub fn set_theme_mode_with_record(mode: ThemeMode, record: bool, cx: &mut App) {
    let before = current_mode(cx);
    let appearance = cx.window_appearance();
    Theme::set_mode(mode.resolve(appearance), cx);
    if cx.has_global::<gpui_kit::component::Theme>() {
        gpui_kit::component::Theme::change(mode.kit_mode(appearance), None, cx);
        corpora_ui::styles::tokens::disable_focus_outlines(cx);
    }
    crate::app_state::update_config(cx, |config| config.theme = mode);
    if record && before != mode {
        crate::undo_stack::record_theme_mode_change(before, mode, cx);
    }
}
