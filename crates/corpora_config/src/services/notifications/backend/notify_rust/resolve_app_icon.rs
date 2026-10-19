#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "linux")]
pub(crate) fn resolve_app_icon() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(probe_app_icon))
        .unwrap_or_else(|| APP_ICON_NAME.to_string())
}
