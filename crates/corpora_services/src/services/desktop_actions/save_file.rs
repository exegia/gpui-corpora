#[allow(unused_imports)]
use super::*;

pub fn save_file(cx: &mut App) -> Option<PathBuf> {
    pick_with_dialog("save_file", rfd::FileDialog::new().save_file(), cx)
}
