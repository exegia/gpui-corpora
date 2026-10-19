#[allow(unused_imports)]
use super::*;

// async_trait's generated wrapper fns carry #[must_use] and return boxed
// futures (already #[must_use]) — new nightly clippy flags the generated
// code as double_must_use. The attribute is macro-emitted, so the allow
// lives here at the macro use site (same as NotificationBackend).
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn schema_version(&self) -> Result<i64, StorageError>;
    async fn health_check(&self) -> Result<(), StorageError>;
    async fn maintenance(&self) -> Result<(), StorageError>;
    async fn persist_error_record(
        &self,
        error: &crate::error_surface::ErrorRecord,
    ) -> Result<(), StorageError>;
    async fn load_error_history(
        &self,
        limit: usize,
    ) -> Result<Vec<crate::error_surface::ErrorRecord>, StorageError>;
    async fn persist_crash_report(
        &self,
        report: &crate::services::crash_report::CrashReport,
    ) -> Result<(), StorageError>;
    async fn load_pending_crash_reports(
        &self,
        limit: usize,
    ) -> Result<Vec<crate::services::crash_report::CrashReport>, StorageError>;
    async fn mark_crash_report_uploaded(
        &self,
        id: &str,
        uploaded_at: &str,
    ) -> Result<(), StorageError>;
}
