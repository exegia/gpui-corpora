#[allow(unused_imports)]
use super::*;

pub fn delete_secret(service: &str, key: &str, cx: &mut App) -> Result<(), SecureStorageError> {
    let entry = entry(service, key)?;
    entry.delete_credential().map_err(|err| {
        tracing::error!(target: "corpora::secure_storage", service, key, "delete failed: {err}");
        update_last_error(Some(err.to_string()), cx);
        SecureStorageError::DeleteFailed {
            service: service.to_string(),
            key: key.to_string(),
            source: err,
        }
    })?;
    tracing::info!(target: "corpora::secure_storage", service, key, "secret deleted");
    update_last_error(None, cx);
    Ok(())
}
