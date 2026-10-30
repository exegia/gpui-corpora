#[allow(unused_imports)]
use super::*;

pub(crate) fn quarantine_bad_config(path: &Path) {
    if !path.exists() {
        return;
    }
    let quarantine_path = path.with_extension("json.bad");
    if let Err(err) = std::fs::rename(path, &quarantine_path) {
        tracing::warn!(
            target: "corpora::app_state",
            source = %path.display(),
            target_path = %quarantine_path.display(),
            error = %err,
            "failed to quarantine corrupt app state"
        );
    }
}
