#[allow(unused_imports)]
use super::*;

pub fn current_mode(cx: &App) -> ThemeMode {
    cx.theme().mode().into()
}
