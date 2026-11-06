#[allow(unused_imports)]
use super::*;

/// App-wide config store (GPUI global). Read through the helper fns in
/// this module; write only via [`update_config`] and [`force_save`],
/// which own the debounced persistence.
pub struct AppState {
    pub(crate) paths: AppPaths,
    pub(crate) config: AppConfig,
    pub(crate) last_load_error: Option<String>,
    pub(crate) last_save_error: Option<String>,
    pub(crate) dirty: bool,
    // Serialized bytes of the last successful flush; lets identical states skip the write.
    pub(crate) last_flushed_bytes: Vec<u8>,
    // Bound debounce task; dropped on shutdown so it cannot commit stale bytes
    // beneath the synchronous flush (dropping a `Task` cancels it).
    pub(crate) in_flight_save: Option<Task<()>>,
}

impl Global for AppState {}
