#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorRecord {
    pub id: EventId,
    pub occurred_at: AppTimestamp,
    pub severity: crate::errors::AppErrorSeverity,
    pub category: ErrorCategory,
    pub message: String,
    pub actions: Vec<ErrorAction>,
}
