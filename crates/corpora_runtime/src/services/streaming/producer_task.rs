#[allow(unused_imports)]
use super::*;

/// Run `pump` on the shared tokio runtime.
pub(crate) fn producer_task(cx: &App, pump: impl Future<Output = ()> + Send + 'static) -> Task<()> {
    let runtime = cx
        .try_global::<crate::services::tokio_runtime::TokioRuntimeGlobal>()
        .map(|g| g.0.runtime.clone());
    cx.spawn(async move |_cx: &mut AsyncApp| {
        let Some(runtime) = runtime else {
            tracing::warn!(
                target: "corpora::streaming",
                "TokioRuntimeGlobal not set; token stream cannot start"
            );
            return;
        };
        runtime.spawn(pump).await.ok();
    })
}
