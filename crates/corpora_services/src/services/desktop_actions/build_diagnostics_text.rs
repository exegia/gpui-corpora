#[allow(unused_imports)]
use super::*;

pub(crate) fn build_diagnostics_text(cx: &App) -> String {
    let app_state = crate::app_state::config(cx);
    let stage = cx
        .try_global::<crate::lifecycle::LifecycleState>()
        .map(|s| s.stage)
        .unwrap_or(crate::lifecycle::LifecycleStage::Starting);
    let connectivity = crate::connectivity::snapshot(cx);
    // CARGO_PKG_NAME here would name this crate, not the app package.
    format!(
        "app={} version={} route={} lifecycle={:?} connectivity={:?}",
        "gpui-corpora",
        env!("CARGO_PKG_VERSION"),
        crate::navigation::to_url(&app_state.active_path),
        stage,
        connectivity.state
    )
}
