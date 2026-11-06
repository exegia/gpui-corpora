#[allow(unused_imports)]
use super::*;

/// Canonicalize `path` and verify it has no `..` components and stays within
/// one of `allowed_dirs`. The file must exist.
pub fn validate_file_path(
    path: &str,
    allowed_dirs: &[PathBuf],
) -> Result<PathBuf, ValidationError> {
    let raw = Path::new(path);

    for component in raw.components() {
        if component == std::path::Component::ParentDir {
            return Err(ValidationError::PathTraversal {
                input: path.to_string(),
            });
        }
    }

    let canonical = raw.canonicalize().map_err(|_| ValidationError::NotFound {
        input: path.to_string(),
    })?;

    let permitted = allowed_dirs.iter().any(|dir| {
        let Ok(canonical_dir) = dir.canonicalize() else {
            return false;
        };
        canonical.starts_with(&canonical_dir)
    });

    if !permitted {
        return Err(ValidationError::EscapesAllowedDir {
            input: path.to_string(),
        });
    }

    Ok(canonical)
}
