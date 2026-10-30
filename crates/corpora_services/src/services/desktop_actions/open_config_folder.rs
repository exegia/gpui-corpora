#[allow(unused_imports)]
use super::*;

pub fn open_config_folder(cx: &mut App) -> Result<(), DesktopActionError> {
    let path = crate::app_state::paths(cx).config_dir;
    open_path(path, cx)
}
