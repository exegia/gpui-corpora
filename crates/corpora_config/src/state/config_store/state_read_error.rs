#[allow(unused_imports)]
use super::*;

pub(crate) fn state_read_error(path: &Path, err: std::io::Error) -> (AppConfig, Option<String>) {
    (
        AppConfig::default(),
        Some(
            AppError::StateRead {
                path: path.to_path_buf(),
                details: err.to_string(),
            }
            .to_string(),
        ),
    )
}
