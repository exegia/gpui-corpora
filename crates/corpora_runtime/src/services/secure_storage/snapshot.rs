#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> SecureStorageSnapshot {
    cx.try_global::<SecureStorageSnapshot>()
        .cloned()
        .unwrap_or_default()
}
