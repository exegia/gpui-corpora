#[allow(unused_imports)]
use super::*;

/// Borrow the active [`AppConfig`] without cloning the whole struct; prefer
/// this (or [`with_config`]) over [`config`] in render paths.
pub fn config_handle(cx: &App) -> Option<&AppConfig> {
    cx.try_global::<AppState>().map(|s| &s.config)
}
