#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> ConnectivitySnapshot {
    cx.try_global::<ConnectivitySnapshot>()
        .cloned()
        .unwrap_or_default()
}
