#[allow(unused_imports)]
use super::*;

pub fn pick_folder(cx: &mut App) -> Option<PathBuf> {
    pick_with_dialog("pick_folder", rfd::FileDialog::new().pick_folder(), cx)
}
