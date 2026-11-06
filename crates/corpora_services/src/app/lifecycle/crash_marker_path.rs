#[allow(unused_imports)]
use super::*;

// Marker lives under the app data dir, never $TMPDIR (symlink planting); an
// unset dir disables the feature.
pub(crate) fn crash_marker_path() -> Option<PathBuf> {
    APP_DATA_DIR.get().map(|dir| dir.join("crash-marker"))
}
