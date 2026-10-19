#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let paths = match AppPaths::new() {
        Ok(paths) => paths,
        Err(err) => {
            tracing::error!(target: "corpora::app_state", error = %err, "failed to initialize app paths");
            return;
        }
    };

    let (config, last_load_error) = load_config(&paths.state_file);
    tracing::info!(
        target: "corpora::app_state",
        state_file = %paths.state_file.display(),
        config_dir = %paths.config_dir.display(),
        data_dir = %paths.data_dir.display(),
        log_dir = %paths.log_dir.display(),
        last_load_error = ?last_load_error,
        "loaded app state"
    );

    // Pre-compute the initial flushed bytes so that a no-op update_config
    // immediately after startup will skip the write.
    let initial_bytes = serde_json::to_vec(&config).unwrap_or_default();

    cx.set_global(AppState {
        paths,
        config,
        last_load_error,
        last_save_error: None,
        dirty: false,
        last_flushed_bytes: initial_bytes,
        in_flight_save: None,
    });
}
