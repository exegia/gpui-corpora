#[allow(unused_imports)]
use super::*;

pub(crate) fn oversized_state_error(path: &Path) -> (AppConfig, Option<String>) {
    (
        AppConfig::default(),
        Some(
            AppError::StateParse {
                path: path.to_path_buf(),
                details: format!("file exceeds the {MAX_STATE_FILE_BYTES}-byte cap"),
            }
            .to_string(),
        ),
    )
}
