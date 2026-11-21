#[allow(unused_imports)]
use super::*;

pub fn pick_epub_file(cx: &mut App) -> Option<PathBuf> {
    pick_with_dialog(
        "pick_epub_file",
        rfd::FileDialog::new()
            .add_filter("EPUB", &["epub"])
            .pick_file(),
        cx,
    )
}
