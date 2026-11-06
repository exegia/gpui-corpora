#[allow(unused_imports)]
use super::*;

/// Write pre-serialized bytes atomically.
pub(crate) fn save_config(path: &Path, json_bytes: &[u8]) -> Result<(), AppError> {
    ensure_parent_dir(path)?;
    // `mut` is only consumed by the unix mode/preserve_mode calls below.
    #[cfg_attr(not(target_family = "unix"), allow(unused_mut))]
    let mut options = AtomicWriteFile::options();
    // The rename commit replaces a planted symlink instead of writing
    // through it; every commit lands 0600 (preserve_mode off).
    #[cfg(target_family = "unix")]
    {
        use atomic_write_file::unix::OpenOptionsExt as _;
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600).preserve_mode(false);
    }
    let mut file = options.open(path).map_err(|err| AppError::StateWrite {
        path: path.to_path_buf(),
        details: err.to_string(),
    })?;
    file.write_all(json_bytes)
        .map_err(|err| AppError::StateWrite {
            path: path.to_path_buf(),
            details: err.to_string(),
        })?;
    file.write_all(b"\n").map_err(|err| AppError::StateWrite {
        path: path.to_path_buf(),
        details: err.to_string(),
    })?;
    file.commit().map_err(|err| AppError::StateWrite {
        path: path.to_path_buf(),
        details: err.to_string(),
    })?;
    Ok(())
}
