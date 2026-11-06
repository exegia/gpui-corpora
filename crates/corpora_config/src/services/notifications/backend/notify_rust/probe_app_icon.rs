#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "linux")]
pub(crate) fn probe_app_icon(exe_dir: &Path) -> String {
    // A name the daemon cannot resolve is a no-op, so the theme name doubles
    // as the fallback for hosts whose hicolor tree carries the icon.
    icon_candidates(exe_dir)
        .into_iter()
        .find(|candidate| candidate.is_file())
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| APP_ICON_NAME.to_string())
}
