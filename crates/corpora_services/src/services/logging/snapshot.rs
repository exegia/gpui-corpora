#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> LoggingRuntime {
    cx.try_global::<LoggingState>()
        .map(|state| state.runtime.clone())
        .unwrap_or(LoggingRuntime {
            enabled: false,
            log_dir: String::new(),
            file_prefix: "corpora.log".to_string(),
            has_guard: false,
            last_error: Some("logging not initialized".to_string()),
        })
}
