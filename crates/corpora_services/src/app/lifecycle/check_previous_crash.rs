#[allow(unused_imports)]
use super::*;

pub fn check_previous_crash() -> Option<String> {
    let path = crash_marker_path()?;
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(contents) => Some(contents),
            Err(err) => {
                tracing::warn!(
                    target: "corpora::lifecycle",
                    path = %path.display(),
                    error = %err,
                    "crash marker exists but could not be read"
                );
                Some("<unreadable>".to_string())
            }
        }
    } else {
        None
    }
}
