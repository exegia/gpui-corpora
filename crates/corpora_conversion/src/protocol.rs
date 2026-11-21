//! Wire types for `corpora sidecar --protocol 1`.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub(crate) const PROTOCOL_VERSION: u64 = 1;
/// Largest frame either side may send, including the trailing newline.
pub(crate) const MAX_FRAME_BYTES: usize = 1024 * 1024;
/// Character slots a job may project. Conversion runs on the user's machine, so
/// this only has to exceed what the core's 500 MiB expanded-content limit allows
/// (about 1.05 billion slots, counting notes twice); that limit stops a book first.
pub(crate) const MAX_SLOTS: u64 = u32::MAX as u64;

/// A message the sidecar writes to standard output.
///
/// Version 1 rejects unknown fields and message types, so decoding is strict.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SidecarMessage {
    Ready {
        capabilities: Vec<String>,
    },
    Accepted {
        request_id: String,
        job_id: String,
        staging_path: PathBuf,
    },
    Progress {
        job_id: String,
        phase: ProgressPhase,
        message: String,
    },
    Diagnostic {
        job_id: String,
        severity: DiagnosticSeverity,
        code: String,
        message: String,
        #[serde(default)]
        location: Option<DiagnosticLocation>,
    },
    Finished {
        job_id: String,
        outcome: JobOutcome,
        outputs: Vec<JobOutput>,
        #[serde(default)]
        error: Option<JobError>,
    },
    CancelAck {
        request_id: String,
        job_id: String,
        status: CancelStatus,
    },
    Error {
        request_id: String,
        code: String,
        message: String,
    },
    ProtocolError {
        code: String,
        message: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressPhase {
    Validating,
    Converting,
    Finalizing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Where a diagnostic points inside the EPUB archive. Lines and columns are one-based.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticLocation {
    #[serde(default)]
    pub member: Option<String>,
    #[serde(default)]
    pub line: Option<u64>,
    #[serde(default)]
    pub column: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobOutcome {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobOutput {
    pub kind: OutputKind,
    pub path: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    TfBundle,
}

/// Why a job failed. `staging_path` is set only for `cleanup_incomplete`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub staging_path: Option<PathBuf>,
}

/// `requested` is the only status: an acknowledgement never claims the worker stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelStatus {
    Requested,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Request<'a> {
    Start {
        protocol: u64,
        request_id: &'a str,
        operation: &'static str,
        input: PathArgument<'a>,
        output: PathArgument<'a>,
        options: StartOptions,
    },
    Cancel {
        protocol: u64,
        request_id: &'a str,
        job_id: &'a str,
    },
}

#[derive(Serialize)]
pub(crate) struct PathArgument<'a> {
    pub path: &'a str,
}

#[derive(Serialize)]
pub(crate) struct StartOptions {
    pub repair: bool,
    pub max_slots: u64,
}

/// Encode a request as one newline-terminated frame.
pub(crate) fn encode(request: &Request<'_>) -> Vec<u8> {
    let mut frame = serde_json::to_vec(request).unwrap_or_default();
    frame.push(b'\n');
    frame
}

/// Decode one frame, rejecting any protocol version other than 1.
pub(crate) fn decode(frame: &[u8]) -> Result<SidecarMessage, String> {
    let mut fields: Map<String, Value> =
        serde_json::from_slice(frame).map_err(|error| format!("invalid JSON frame: {error}"))?;
    match fields.remove("protocol") {
        Some(Value::Number(version)) if version.as_u64() == Some(PROTOCOL_VERSION) => {}
        other => return Err(format!("unsupported protocol version: {other:?}")),
    }
    serde_json::from_value(Value::Object(fields))
        .map_err(|error| format!("invalid sidecar message: {error}"))
}

/// The sidecar takes absolute local paths, and JSON needs them as UTF-8.
pub(crate) fn protocol_path(path: &Path) -> Option<&str> {
    path.is_absolute().then(|| path.to_str()).flatten()
}
