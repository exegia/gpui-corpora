#[allow(unused_imports)]
use super::*;

pub fn current_locale(cx: &App) -> SharedString {
    cx.global::<LocaleState>().0.clone()
}
