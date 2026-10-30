#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> StorageSnapshot {
    cx.try_global::<StorageSnapshot>()
        .cloned()
        .unwrap_or_default()
}
