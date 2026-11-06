#[allow(unused_imports)]
use super::*;

/// App-owned downloads + swap directory under the user's data dir, so the
/// write target is not a world-shared, attacker-creatable temp path.
/// `None` (no data dir / cannot create it) fails closed.
pub(crate) fn updates_dir() -> Option<PathBuf> {
    let dir = crate::platform::filesystem::paths::project_dirs()?
        .data_dir()
        .join("updates");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}
