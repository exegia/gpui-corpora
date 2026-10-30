#[allow(unused_imports)]
use super::*;

pub fn set_theme_mode(mode: ThemeMode, cx: &mut App) {
    set_theme_mode_with_record(mode, true, cx);
}
