use std::{fs, path::PathBuf};

use super::{EpubJob, JobResult, JobUpdate, next_output_path};
use crate::{
    CloseReason, DiagnosticLocation, DiagnosticSeverity, JobError, JobOutcome, JobOutput,
    OutputKind, ProgressPhase, SidecarEvent, SidecarMessage,
};

fn accepted(request_id: &str, job_id: &str) -> SidecarEvent {
    SidecarEvent::Message(SidecarMessage::Accepted {
        request_id: request_id.into(),
        job_id: job_id.into(),
        staging_path: "/books/.out-job-1".into(),
    })
}

fn finished(job_id: &str, outcome: JobOutcome, error: Option<JobError>) -> SidecarEvent {
    let outputs = match outcome {
        JobOutcome::Succeeded => vec![JobOutput {
            kind: OutputKind::TfBundle,
            path: "/books/out".into(),
        }],
        _ => Vec::new(),
    };
    SidecarEvent::Message(SidecarMessage::Finished {
        job_id: job_id.into(),
        outcome,
        outputs,
        error,
    })
}

#[test]
fn follows_a_successful_job_and_ignores_other_requests() {
    let mut job = EpubJob::new("req-1".into());

    assert_eq!(job.apply(accepted("req-9", "job-9")), JobUpdate::Ignored);
    assert_eq!(job.apply(accepted("req-1", "job-1")), JobUpdate::Accepted);
    assert_eq!(job.job_id(), Some("job-1"));
    assert_eq!(
        job.apply(SidecarEvent::Message(SidecarMessage::Progress {
            job_id: "job-1".into(),
            phase: ProgressPhase::Converting,
            message: "Writing Text-Fabric".into(),
        })),
        JobUpdate::Progress(ProgressPhase::Converting)
    );
    assert_eq!(
        job.apply(SidecarEvent::Message(SidecarMessage::Diagnostic {
            job_id: "job-1".into(),
            severity: DiagnosticSeverity::Warning,
            code: "EXAMPLE".into(),
            message: "Review".into(),
            location: None,
        })),
        JobUpdate::Diagnostic {
            severity: DiagnosticSeverity::Warning,
            code: "EXAMPLE".into(),
        }
    );
    assert_eq!(
        job.apply(finished("job-9", JobOutcome::Failed, None)),
        JobUpdate::Ignored
    );
    assert_eq!(
        job.apply(finished("job-1", JobOutcome::Succeeded, None)),
        JobUpdate::Finished(JobResult::Succeeded {
            output: "/books/out".into(),
        })
    );
}

#[test]
fn fails_when_the_start_request_is_rejected() {
    let mut job = EpubJob::new("req-1".into());

    let update = job.apply(SidecarEvent::Message(SidecarMessage::Error {
        request_id: "req-1".into(),
        code: "output_exists".into(),
        message: "The output directory already exists".into(),
    }));

    assert_eq!(
        update,
        JobUpdate::Finished(JobResult::Failed {
            code: "output_exists".into(),
            message: "The output directory already exists".into(),
            issues: Vec::new(),
            more_issues: 0,
            staging_path: None,
        })
    );
}

fn error_diagnostic(member: Option<&str>, message: &str) -> SidecarEvent {
    SidecarEvent::Message(SidecarMessage::Diagnostic {
        job_id: "job-1".into(),
        severity: DiagnosticSeverity::Error,
        code: "EPUB_XML".into(),
        message: message.into(),
        location: member.map(|member| DiagnosticLocation {
            member: Some(member.into()),
            line: None,
            column: None,
        }),
    })
}

#[test]
fn reports_a_bounded_sample_of_distinct_blocking_issues() {
    let mut job = EpubJob::new("req-1".into());
    job.apply(accepted("req-1", "job-1"));
    job.apply(error_diagnostic(Some("OEBPS/a.xhtml"), "Malformed XML"));
    job.apply(error_diagnostic(Some("OEBPS/a.xhtml"), "Malformed XML"));
    job.apply(error_diagnostic(Some("<archive>"), "Too large"));
    job.apply(error_diagnostic(None, "No reading content"));
    job.apply(SidecarEvent::Message(SidecarMessage::Diagnostic {
        job_id: "job-1".into(),
        severity: DiagnosticSeverity::Warning,
        code: "EPUB_STYLE".into(),
        message: "Review styling".into(),
        location: None,
    }));
    job.apply(error_diagnostic(Some("OEBPS/b.xhtml"), "Malformed XML"));
    job.apply(error_diagnostic(Some("OEBPS/c.xhtml"), "Malformed XML"));

    let update = job.apply(finished(
        "job-1",
        JobOutcome::Failed,
        Some(JobError {
            code: "invalid_epub".into(),
            message: "EPUB validation found blocking issues.".into(),
            staging_path: None,
        }),
    ));

    assert_eq!(
        update,
        JobUpdate::Finished(JobResult::Failed {
            code: "invalid_epub".into(),
            message: "EPUB validation found blocking issues.".into(),
            issues: vec![
                "OEBPS/a.xhtml: Malformed XML".into(),
                "Too large".into(),
                "No reading content".into(),
            ],
            more_issues: 2,
            staging_path: None,
        })
    );
}

#[test]
fn keeps_the_staging_path_of_an_incomplete_cleanup() {
    let mut job = EpubJob::new("req-1".into());
    job.apply(accepted("req-1", "job-1"));

    let update = job.apply(finished(
        "job-1",
        JobOutcome::Failed,
        Some(JobError {
            code: "cleanup_incomplete".into(),
            message: "Cleanup failed".into(),
            staging_path: Some("/books/.out-job-1".into()),
        }),
    ));

    assert_eq!(
        update,
        JobUpdate::Finished(JobResult::Failed {
            code: "cleanup_incomplete".into(),
            message: "Cleanup failed".into(),
            issues: Vec::new(),
            more_issues: 0,
            staging_path: Some("/books/.out-job-1".into()),
        })
    );
}

#[test]
fn reports_cancellation() {
    let mut job = EpubJob::new("req-1".into());
    job.apply(accepted("req-1", "job-1"));

    assert_eq!(
        job.apply(finished("job-1", JobOutcome::Cancelled, None)),
        JobUpdate::Finished(JobResult::Cancelled)
    );
}

#[test]
fn reports_an_unknown_outcome_with_the_staging_path_when_the_stream_closes() {
    let mut job = EpubJob::new("req-1".into());
    job.apply(accepted("req-1", "job-1"));

    assert_eq!(
        job.apply(SidecarEvent::Closed(CloseReason::EndOfStream)),
        JobUpdate::Finished(JobResult::Unknown {
            staging_path: Some("/books/.out-job-1".into()),
        })
    );
}

#[test]
fn picks_the_first_output_name_that_does_not_exist() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let input = directory.path().join("field-guide.epub");

    assert_eq!(
        next_output_path(&input),
        Some(directory.path().join("field-guide-tf"))
    );

    fs::create_dir(directory.path().join("field-guide-tf")).expect("create first output");
    assert_eq!(
        next_output_path(&input),
        Some(directory.path().join("field-guide-tf-2"))
    );
}

#[test]
fn has_no_output_path_without_a_parent_directory() {
    assert_eq!(next_output_path(&PathBuf::from("/")), None);
}
