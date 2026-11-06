#[allow(unused_imports)]
use super::*;

/// Plain free fn, safe to call
/// before any GPUI `App` exists (e.g. single-instance preflight).
pub fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "corpora", "Corpora")
}
