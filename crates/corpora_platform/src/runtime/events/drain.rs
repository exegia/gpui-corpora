#[allow(unused_imports)]
use super::*;

pub fn drain(cx: &mut App) -> Vec<AppEvent> {
    // Read-only early return: default_global() mutates on every call, which
    // re-fires observe_global callbacks whose drain() would loop forever.
    if cx
        .try_global::<AppEventQueue>()
        .map_or(true, |q| q.0.is_empty())
    {
        return Vec::new();
    }
    std::mem::take(&mut cx.default_global::<AppEventQueue>().0)
}
