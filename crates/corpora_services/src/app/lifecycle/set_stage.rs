#[allow(unused_imports)]
use super::*;

pub fn set_stage(stage: LifecycleStage, cx: &mut App) {
    let state = cx.default_global::<LifecycleState>();
    state.stage = stage;
    state.updated_at = AppTimestamp::now();
}
