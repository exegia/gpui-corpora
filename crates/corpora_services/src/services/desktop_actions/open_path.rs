#[allow(unused_imports)]
use super::*;

pub fn open_path(path: PathBuf, cx: &mut App) -> Result<(), DesktopActionError> {
    let result =
        open::that_detached(path.as_path()).map_err(|source| DesktopActionError::OpenPathFailed {
            path: path.display().to_string(),
            source,
        });
    update_result(
        "open_path",
        result.as_ref().err().map(|e| e.to_string()),
        cx,
    );
    result
}
