#[allow(unused_imports)]
use super::*;

pub(crate) fn entry(service: &str, key: &str) -> Result<keyring::Entry, SecureStorageError> {
    keyring::Entry::new(service, key).map_err(|err| {
        tracing::error!(target: "corpora::secure_storage", "entry creation failed: {err}");
        SecureStorageError::EntryCreation {
            service: service.to_string(),
            key: key.to_string(),
            source: err,
        }
    })
}
