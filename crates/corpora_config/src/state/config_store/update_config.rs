#[allow(unused_imports)]
use super::*;

/// Mutate the config and schedule a debounced, coalesced save; the closure
/// runs synchronously. Use [`force_save`] for immediate persistence.
pub fn update_config(cx: &mut App, update: impl FnOnce(&mut AppConfig)) {
    if cx.try_global::<AppState>().is_none() {
        tracing::warn!(target: "corpora::app_state", "attempted to update app state before initialization");
        return;
    }

    cx.update_global::<AppState, _>(|state, _cx| {
        update(&mut state.config);
        state.config = state.config.clone().normalized();
        state.dirty = true;
    });

    arm_debounce(cx);
}
