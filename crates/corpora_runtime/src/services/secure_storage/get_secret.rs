#[allow(unused_imports)]
use super::*;

pub fn get_secret(
    service: &str,
    key: &str,
    cx: &mut App,
) -> Result<Option<SharedString>, SecureStorageError> {
    let entry = entry(service, key)?;
    match entry.get_password() {
        Ok(value) => {
            tracing::info!(target: "corpora::secure_storage", service, key, "secret read");
            update_last_error(None, cx);
            Ok(Some(value.into()))
        }
        Err(keyring::Error::NoEntry) => {
            tracing::warn!(target: "corpora::secure_storage", service, key, "no entry found");
            Ok(None)
        }
        Err(err) => {
            tracing::error!(target: "corpora::secure_storage", service, key, "get_password failed: {err}");
            Err(SecureStorageError::GetFailed {
                service: service.to_string(),
                key: key.to_string(),
                source: err,
            })
        }
    }
}
