#[allow(unused_imports)]
use super::*;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("path traversal detected in `{input}`")]
    PathTraversal { input: String },

    #[error("path `{input}` escapes allowed directory")]
    EscapesAllowedDir { input: String },

    #[error("path does not exist: `{input}`")]
    NotFound { input: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
