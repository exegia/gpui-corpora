#[allow(unused_imports)]
use super::*;

pub fn open_logs_folder(cx: &mut App) -> Result<(), DesktopActionError> {
    let path = crate::app_state::paths(cx).log_dir;
    open_path(path, cx)
}
