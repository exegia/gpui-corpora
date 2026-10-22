#[allow(unused_imports)]
use super::*;

pub fn set_secret(
    service: &str,
    key: &str,
    value: &str,
    cx: &mut App,
) -> Result<(), SecureStorageError> {
    let entry = entry(service, key)?;
    entry.set_password(value).map_err(|err| {
        tracing::error!(target: "corpora::secure_storage", service, key, "set_password failed: {err}");
        update_last_error(Some(err.to_string()), cx);
        SecureStorageError::SetFailed { service: service.to_string(), key: key.to_string(), source: err }
    })?;
    tracing::info!(target: "corpora::secure_storage", service, key, "secret written");
    update_last_error(None, cx);
    Ok(())
}
