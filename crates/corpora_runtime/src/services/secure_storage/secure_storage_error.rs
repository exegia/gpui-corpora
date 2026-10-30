#[allow(unused_imports)]
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum SecureStorageError {
    #[error("entry creation failed for '{service}/{key}': {source}")]
    EntryCreation {
        service: String,
        key: String,
        #[source]
        source: keyring::Error,
    },
    #[error("failed to set secret for '{service}/{key}': {source}")]
    SetFailed {
        service: String,
        key: String,
        #[source]
        source: keyring::Error,
    },
    #[error("failed to get secret for '{service}/{key}': {source}")]
    GetFailed {
        service: String,
        key: String,
        #[source]
        source: keyring::Error,
    },
    #[error("failed to delete secret for '{service}/{key}': {source}")]
    DeleteFailed {
        service: String,
        key: String,
        #[source]
        source: keyring::Error,
    },
}
