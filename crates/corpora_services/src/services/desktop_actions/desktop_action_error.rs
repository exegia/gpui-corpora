#[allow(unused_imports)]
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum DesktopActionError {
    #[error("clipboard operation failed: {0}")]
    Clipboard(#[from] arboard::Error),
    #[error("failed to open path '{path}'")]
    OpenPathFailed {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to open url '{url}'")]
    OpenUrlFailed {
        url: String,
        #[source]
        source: std::io::Error,
    },
    #[error("file dialog failed: {0}")]
    DialogFailed(String),
    #[error("watcher error: {0}")]
    Watcher(#[from] notify::Error),
    #[error("desktop actions unavailable")]
    Unavailable,
    #[error("watcher lock poisoned")]
    LockPoisoned,
}
