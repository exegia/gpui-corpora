#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn error_record_serializes_with_category() {
    let record = ErrorRecord {
        id: EventId::new(),
        occurred_at: AppTimestamp::now(),
        severity: crate::errors::AppErrorSeverity::Warning,
        category: ErrorCategory::Network,
        message: "connection timed out".into(),
        actions: vec![ErrorAction::Retry, ErrorAction::Dismiss],
    };
    let json = serde_json::to_string(&record).expect("serialize");
    assert!(json.contains("Network"));
    let deserialized: ErrorRecord = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deserialized.category, ErrorCategory::Network);
}
