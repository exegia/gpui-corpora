#[allow(unused_imports)]
use super::*;

/// Callers should degrade gracefully when this returns `None`.
pub fn runtime_and_client(cx: &App) -> Option<(Arc<tokio::runtime::Runtime>, reqwest::Client)> {
    cx.try_global::<TokioRuntimeGlobal>()
        .map(|g| (g.0.runtime.clone(), g.0.http_client.clone()))
}
