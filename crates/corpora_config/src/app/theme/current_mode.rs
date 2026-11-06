#[allow(unused_imports)]
use super::*;

pub fn current_mode(cx: &App) -> ThemeMode {
    crate::app_state::with_config(cx, |config| config.theme)
}
