#[allow(unused_imports)]
use super::*;

/// Callers should degrade gracefully when this returns `None`.
pub fn handle(cx: &App) -> Option<Arc<tokio::runtime::Runtime>> {
    cx.try_global::<TokioRuntimeGlobal>()
        .map(|g| g.0.runtime.clone())
}
