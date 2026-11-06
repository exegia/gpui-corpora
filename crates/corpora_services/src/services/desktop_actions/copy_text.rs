#[allow(unused_imports)]
use super::*;

pub fn copy_text(text: &str, cx: &mut App) -> Result<(), DesktopActionError> {
    let result = Clipboard::new()
        .and_then(|mut clipboard| clipboard.set_text(text.to_string()))
        .map_err(DesktopActionError::from);
    update_result(
        "clipboard_copy",
        result.as_ref().err().map(|e| e.to_string()),
        cx,
    );
    result
}
