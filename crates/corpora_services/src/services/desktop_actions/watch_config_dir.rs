#[allow(unused_imports)]
use super::*;

pub fn watch_config_dir(cx: &mut App) -> Result<u64, DesktopActionError> {
    let path = crate::app_state::paths(cx).config_dir;
    watch_path(path, cx)
}
