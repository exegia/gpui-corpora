#[allow(unused_imports)]
use super::*;

/// `Err` is a recoverable transport/IO failure routed to retry/backoff;
/// `PermanentFailure` covers verification failures, which must never retry.
pub(crate) enum DownloadOutcome {
    Success { version: String, path: String },
    PermanentFailure(String),
}
