#[allow(unused_imports)]
use super::*;

/// Persist a single error record to the `error_log` table. If the table does
/// not exist yet this fails and the in-memory store remains authoritative.
pub(crate) async fn persist_error(
    db: &dyn crate::storage::StorageBackend,
    error: &ErrorRecord,
) -> Result<(), crate::storage::StorageError> {
    db.persist_error_record(error).await
}
