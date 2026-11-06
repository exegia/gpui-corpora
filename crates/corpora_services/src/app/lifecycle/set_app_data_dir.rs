#[allow(unused_imports)]
use super::*;

/// Point the panic hook at the app data dir; call once, after app_state init.
pub fn set_app_data_dir(path: PathBuf) {
    let _ = APP_DATA_DIR.set(path);
}
