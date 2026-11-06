#[allow(unused_imports)]
use super::*;

pub fn set_startup_error(error: impl Into<String>, cx: &mut App) {
    let error = error.into();
    let state = cx.default_global::<LifecycleState>();
    state.last_startup_error = Some(error.clone());
    state.last_error = Some(error);
    state.updated_at = AppTimestamp::now();
}
