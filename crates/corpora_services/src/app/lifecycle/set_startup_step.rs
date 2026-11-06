#[allow(unused_imports)]
use super::*;

pub fn set_startup_step(step: impl Into<String>, cx: &mut App) {
    let state = cx.default_global::<LifecycleState>();
    state.startup_step = Some(step.into());
    state.updated_at = AppTimestamp::now();
}
