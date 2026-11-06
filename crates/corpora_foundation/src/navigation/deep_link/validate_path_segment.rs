#[allow(unused_imports)]
use super::*;

pub(crate) fn validate_path_segment(segment: &str) -> Result<(), AppError> {
    if segment.is_empty() {
        return Ok(());
    }

    if segment.chars().any(char::is_control) {
        return Err(AppError::invalid_deep_link(
            segment.escape_default().to_string(),
            "path segment contains a control character",
        ));
    }

    if segment == ".." || segment == "." || segment.contains("..") {
        return Err(AppError::invalid_deep_link(
            segment,
            "path traversal detected in URL segment",
        ));
    }

    for &forbidden in INVALID_SEGMENT_CHARS {
        if segment.contains(forbidden) {
            return Err(AppError::invalid_deep_link(
                segment,
                format!("path segment contains forbidden character `{forbidden:?}`"),
            ));
        }
    }

    Ok(())
}
