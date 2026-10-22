#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> SessionSnapshot {
    cx.try_global::<SessionSnapshot>()
        .cloned()
        .unwrap_or_default()
}
