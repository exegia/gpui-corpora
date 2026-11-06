#[allow(unused_imports)]
use super::*;

/// Run a closure with borrowed access to the active [`AppConfig`], falling
/// back to the default when [`initialize`] has not run.
pub fn with_config<R>(cx: &App, f: impl FnOnce(&AppConfig) -> R) -> R {
    match cx.try_global::<AppState>() {
        Some(state) => f(&state.config),
        None => f(&AppConfig::default()),
    }
}
