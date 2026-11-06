#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> ShortcutState {
    cx.try_global::<ShortcutState>()
        .cloned()
        .unwrap_or_default()
}
