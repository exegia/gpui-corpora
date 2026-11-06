#[allow(unused_imports)]
use super::*;

pub fn write_crash_marker() {
    let Some(path) = crash_marker_path() else {
        return;
    };
    let pid = std::process::id();
    let timestamp = chrono::Utc::now().to_rfc3339();
    // Unlink first, then create_new 0600: a marker left by an older build or
    // a symlink planted on the path is replaced, never written through.
    let _ = fs::remove_file(&path);
    let mut options = fs::File::options();
    options.write(true).create_new(true);
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    if let Err(err) = options
        .open(&path)
        .and_then(|mut file| write!(file, "pid={pid}\nstarted_at={timestamp}\n"))
    {
        tracing::warn!(
            target: "corpora::lifecycle",
            path = %path.display(),
            error = %err,
            "failed to write crash marker"
        );
    }
}
