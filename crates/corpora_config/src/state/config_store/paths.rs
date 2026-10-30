#[allow(unused_imports)]
use super::*;

pub fn paths(cx: &App) -> AppPaths {
    cx.try_global::<AppState>()
        .map(|s| s.paths.clone())
        .unwrap_or_else(|| {
            tracing::error!(target: "corpora::app_state", "AppState not initialized, using fallback paths");
            AppPaths::new().expect("failed to initialize fallback app paths")
        })
}
