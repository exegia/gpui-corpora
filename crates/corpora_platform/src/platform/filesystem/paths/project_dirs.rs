#[allow(unused_imports)]
use super::*;

/// Plain free fn, safe to call
/// before any GPUI `App` exists (e.g. single-instance preflight).
pub fn project_dirs() -> Option<ProjectDirs> {
    let name = if std::env::var_os("CORPORA_QA_MODE").is_some() {
        "Corpora QA"
    } else {
        "Corpora"
    };
    ProjectDirs::from("com", "corpora", name)
}
