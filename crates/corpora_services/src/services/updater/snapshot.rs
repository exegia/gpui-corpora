#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &gpui::App) -> UpdateSnapshot {
    cx.try_global::<UpdateSnapshot>()
        .cloned()
        .unwrap_or_default()
}
