#[allow(unused_imports)]
use super::*;

/// Point the once-set data dir at a shared temp dir for the marker tests.
pub(crate) fn test_data_dir() -> &'static PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = tempfile::tempdir().expect("tempdir").keep();
        set_app_data_dir(dir.clone());
        dir
    })
}
