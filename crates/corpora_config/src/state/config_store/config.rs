#[allow(unused_imports)]
use super::*;

pub fn config(cx: &App) -> AppConfig {
    cx.try_global::<AppState>()
        .map(|s| s.config.clone())
        .unwrap_or_default()
}
