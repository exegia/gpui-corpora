#[allow(unused_imports)]
use super::*;

/// Hand-built against a throwaway dir because [`initialize`] would point
/// the store at the real OS config dir; the child module reaches the
/// private fields directly.
pub(crate) fn dirty_state(state_file: &std::path::Path) -> AppState {
    let dir = state_file.parent().unwrap().to_path_buf();
    AppState {
        paths: AppPaths {
            config_dir: dir.clone(),
            data_dir: dir.clone(),
            cache_dir: dir.clone(),
            log_dir: dir.clone(),
            runtime_dir: dir,
            state_file: state_file.to_path_buf(),
        },
        config: AppConfig::default(),
        last_load_error: None,
        last_save_error: None,
        dirty: true,
        last_flushed_bytes: Vec::new(),
        in_flight_save: None,
    }
}
