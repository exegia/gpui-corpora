#[allow(unused_imports)]
use super::*;

pub fn remove_crash_marker() {
    let Some(path) = crash_marker_path() else {
        return;
    };
    if path.exists()
        && let Err(err) = fs::remove_file(&path)
    {
        tracing::warn!(
            target: "corpora::lifecycle",
            path = %path.display(),
            error = %err,
            "failed to remove crash marker"
        );
    }
}
