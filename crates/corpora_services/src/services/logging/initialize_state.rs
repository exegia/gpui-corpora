#[allow(unused_imports)]
use super::*;

pub(crate) fn initialize_state(
    cx: &mut App,
    log_dir: String,
    file_prefix: String,
    init_result: Result<(), tracing_subscriber::util::TryInitError>,
    guard: Option<WorkerGuard>,
) {
    let runtime = match init_result {
        Ok(()) => {
            tracing::info!(
                target: "corpora::logging",
                log_dir = %log_dir,
                file_prefix = %file_prefix,
                "logging initialized"
            );
            LoggingRuntime {
                enabled: true,
                log_dir,
                file_prefix,
                has_guard: true,
                last_error: None,
            }
        }
        Err(err) => LoggingRuntime {
            enabled: false,
            log_dir,
            file_prefix,
            has_guard: true,
            last_error: Some(err.to_string()),
        },
    };

    crate::capabilities::set(
        "file_logging",
        crate::capabilities::CapabilityStatus {
            supported: true,
            enabled: runtime.enabled,
            degraded: runtime.last_error.is_some(),
            reason: runtime
                .last_error
                .as_ref()
                .map(|error| format!("logging init failed: {error}").into()),
            last_error: runtime.last_error.clone().map(Into::into),
        },
        cx,
    );

    cx.set_global(LoggingState { runtime, guard });
}
