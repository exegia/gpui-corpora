#[allow(unused_imports)]
use super::*;

pub fn pick_file(cx: &mut App) -> Option<PathBuf> {
    pick_with_dialog("pick_file", rfd::FileDialog::new().pick_file(), cx)
}
