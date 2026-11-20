//! Track one EPUB conversion request through the sidecar event stream.

use std::path::{Path, PathBuf};

use crate::{
    DiagnosticLocation, DiagnosticSeverity, JobError, JobOutcome, JobOutput, OutputKind,
    ProgressPhase, SidecarEvent, SidecarMessage,
};

const OUTPUT_SUFFIX: &str = "-tf";
const MAX_OUTPUT_ATTEMPTS: u32 = 100;
const MAX_REPORTED_ISSUES: usize = 3;
const ARCHIVE_MEMBER: &str = "<archive>";

/// How a conversion request ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobResult {
    Succeeded {
        output: PathBuf,
    },
    /// The sidecar rejected the request or the job failed. `issues` holds the first
    /// distinct error diagnostics and `more_issues` counts the rest. `staging_path`
    /// is set when cleanup could not be confirmed.
    Failed {
        code: String,
        message: String,
        issues: Vec<String>,
        more_issues: usize,
        staging_path: Option<PathBuf>,
    },
    Cancelled,
    /// The sidecar stopped before reporting a result. Files may remain at
    /// `staging_path`; do not delete them or retry into the same destination.
    Unknown {
        staging_path: Option<PathBuf>,
    },
}

/// What one sidecar event meant for the tracked job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JobUpdate {
    Accepted,
    Progress(ProgressPhase),
    Diagnostic {
        severity: DiagnosticSeverity,
        code: String,
    },
    Finished(JobResult),
    /// The event belongs to another request or carries no state for this job.
    Ignored,
}

/// State for a single `start` request, from send until its terminal result.
#[derive(Clone, Debug)]
pub struct EpubJob {
    request_id: String,
    job_id: Option<String>,
    staging_path: Option<PathBuf>,
    issues: Vec<String>,
    more_issues: usize,
}

impl EpubJob {
    /// Track the request whose ID `Sidecar::start_epub_to_tf` returned.
    pub fn new(request_id: String) -> Self {
        Self {
            request_id,
            job_id: None,
            staging_path: None,
            issues: Vec::new(),
            more_issues: 0,
        }
    }

    /// The sidecar's job ID, known once the request is accepted.
    pub fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Apply one event in stream order.
    pub fn apply(&mut self, event: SidecarEvent) -> JobUpdate {
        let message = match event {
            SidecarEvent::Message(message) => message,
            SidecarEvent::Closed(_) => return self.unknown(),
        };
        match message {
            SidecarMessage::Accepted {
                request_id,
                job_id,
                staging_path,
            } if request_id == self.request_id => {
                self.job_id = Some(job_id);
                self.staging_path = Some(staging_path);
                JobUpdate::Accepted
            }
            SidecarMessage::Error {
                request_id,
                code,
                message,
            } if request_id == self.request_id => {
                JobUpdate::Finished(self.failed(code, message, None))
            }
            SidecarMessage::Progress { job_id, phase, .. } if self.is_mine(&job_id) => {
                JobUpdate::Progress(phase)
            }
            SidecarMessage::Diagnostic {
                job_id,
                severity,
                code,
                message,
                location,
            } if self.is_mine(&job_id) => {
                if severity == DiagnosticSeverity::Error {
                    self.record_issue(issue_text(&message, location.as_ref()));
                }
                JobUpdate::Diagnostic { severity, code }
            }
            SidecarMessage::Finished {
                job_id,
                outcome,
                outputs,
                error,
            } if self.is_mine(&job_id) => {
                JobUpdate::Finished(self.finished(outcome, outputs, error))
            }
            // The sidecar closes after a protocol error, so no result will follow.
            SidecarMessage::ProtocolError { .. } => self.unknown(),
            _ => JobUpdate::Ignored,
        }
    }

    fn is_mine(&self, job_id: &str) -> bool {
        self.job_id.as_deref() == Some(job_id)
    }

    fn unknown(&self) -> JobUpdate {
        JobUpdate::Finished(JobResult::Unknown {
            staging_path: self.staging_path.clone(),
        })
    }

    // Keeps a bounded, de-duplicated sample; a large report must not grow this list.
    fn record_issue(&mut self, issue: String) {
        if self.issues.contains(&issue) {
            return;
        }
        if self.issues.len() < MAX_REPORTED_ISSUES {
            self.issues.push(issue);
        } else {
            self.more_issues += 1;
        }
    }

    fn failed(
        &mut self,
        code: String,
        message: String,
        staging_path: Option<PathBuf>,
    ) -> JobResult {
        JobResult::Failed {
            code,
            message,
            issues: std::mem::take(&mut self.issues),
            more_issues: self.more_issues,
            staging_path,
        }
    }

    fn finished(
        &mut self,
        outcome: JobOutcome,
        outputs: Vec<JobOutput>,
        error: Option<JobError>,
    ) -> JobResult {
        match outcome {
            JobOutcome::Succeeded => {
                match outputs
                    .into_iter()
                    .find(|output| output.kind == OutputKind::TfBundle)
                {
                    Some(output) => JobResult::Succeeded {
                        output: output.path,
                    },
                    None => self.failed(
                        "missing_output".into(),
                        "The sidecar reported success without a Text-Fabric bundle.".into(),
                        None,
                    ),
                }
            }
            JobOutcome::Failed => {
                let error = error.unwrap_or_else(|| JobError {
                    code: "unknown_error".into(),
                    message: "The conversion failed.".into(),
                    staging_path: None,
                });
                self.failed(error.code, error.message, error.staging_path)
            }
            JobOutcome::Cancelled => JobResult::Cancelled,
        }
    }
}

fn issue_text(message: &str, location: Option<&DiagnosticLocation>) -> String {
    match location.and_then(|location| location.member.as_deref()) {
        Some(member) if member != ARCHIVE_MEMBER => format!("{member}: {message}"),
        _ => message.to_string(),
    }
}

/// A destination beside `input` that does not exist yet: `book-tf`, then `book-tf-2`, ...
///
/// The sidecar still reserves the destination atomically; this only avoids an
/// avoidable `output exists` failure.
pub fn next_output_path(input: &Path) -> Option<PathBuf> {
    let parent = input.parent()?;
    let stem = input.file_stem()?.to_str()?;
    (1..=MAX_OUTPUT_ATTEMPTS)
        .map(|attempt| match attempt {
            1 => parent.join(format!("{stem}{OUTPUT_SUFFIX}")),
            _ => parent.join(format!("{stem}{OUTPUT_SUFFIX}-{attempt}")),
        })
        // symlink_metadata also sees dangling symlinks, which `exists` reports as absent.
        .find(|candidate| candidate.symlink_metadata().is_err())
}

#[cfg(test)]
#[path = "job.test.rs"]
mod tests;
