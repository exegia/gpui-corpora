#[allow(unused_imports)]
use super::*;

use std::{
    path::{Path, PathBuf},
    pin::pin,
};

use corpora_conversion::{
    CloseReason, EpubJob, EpubTools, JobResult, JobUpdate, Sidecar, SidecarEvent, next_output_path,
};
use futures_util::future::{Either, select};

const READY_TIMEOUT: Duration = Duration::from_secs(30);
// The event wait wakes this often so a shutdown request can cancel the job.
const SHUTDOWN_POLL: Duration = Duration::from_millis(200);
const SIDECAR_GRACE: Duration = Duration::from_secs(10);

pub(crate) fn spawn_epub_conversion(id: TaskId, input: PathBuf, name: String, cx: &mut App) {
    cx.spawn(async move |cx| {
        let background = cx.background_executor().clone();
        let launched = background.spawn(async move { launch(&input) }).await;
        let (mut sidecar, mut job) = match launched {
            Ok(launched) => launched,
            Err(message) => {
                cx.update(|cx| {
                    finish(
                        id,
                        &name,
                        JobResult::Failed {
                            code: "launch_failed".into(),
                            message,
                            issues: Vec::new(),
                            more_issues: 0,
                            staging_path: None,
                        },
                        cx,
                    )
                });
                return;
            }
        };

        let events = sidecar.events().clone();
        let mut cancel_sent = false;
        let result = loop {
            let received = match select(
                pin!(events.recv_async()),
                pin!(background.timer(SHUTDOWN_POLL)),
            )
            .await
            {
                Either::Left((received, _)) => Some(received),
                Either::Right(_) => None,
            };
            let Some(received) = received else {
                if !cancel_sent
                    && is_shutdown_requested()
                    && let Some(job_id) = job.job_id()
                {
                    cancel_sent = sidecar.cancel(job_id).is_ok();
                }
                continue;
            };
            let event = received.unwrap_or(SidecarEvent::Closed(CloseReason::EndOfStream));
            match job.apply(event) {
                JobUpdate::Finished(result) => break result,
                JobUpdate::Progress(phase) => cx.update(|cx| {
                    EpubConversionNotification::Running {
                        name: &name,
                        phase: Some(phase),
                    }
                    .push(id, cx)
                }),
                JobUpdate::Diagnostic { severity, code } => tracing::debug!(
                    target: "corpora::tasks",
                    task_id = %id,
                    ?severity,
                    %code,
                    "EPUB conversion diagnostic"
                ),
                JobUpdate::Accepted | JobUpdate::Ignored => {}
            }
        };

        cx.update(|cx| finish(id, &name, result, cx));
        background
            .spawn(async move {
                if let Err(error) = sidecar.shutdown(SIDECAR_GRACE) {
                    tracing::warn!(target: "corpora::tasks", %error, "EPUB sidecar shutdown failed");
                }
            })
            .detach();
    })
    .detach();
}

fn launch(input: &Path) -> Result<(Sidecar, EpubJob), String> {
    let output =
        next_output_path(input).ok_or_else(|| crate::i18n::localize("tasks_epub_no_output"))?;
    let tools = EpubTools::discover().map_err(|error| {
        tracing::warn!(target: "corpora::tasks", %error, "EPUB runtime unavailable");
        crate::i18n::localize("tasks_epub_runtime_missing")
    })?;
    let mut sidecar = Sidecar::spawn(&tools, READY_TIMEOUT).map_err(|error| error.to_string())?;
    // Repair runs only when validation finds blockers, and the bundle keeps the original.
    let request_id = sidecar
        .start_epub_to_tf(input, &output, true)
        .map_err(|error| error.to_string())?;
    Ok((sidecar, EpubJob::new(request_id)))
}

fn finish(id: TaskId, name: &str, result: JobResult, cx: &mut App) {
    tracing::info!(target: "corpora::tasks", task_id = %id, result = ?result_kind(&result), "EPUB conversion finished");
    match result {
        JobResult::Succeeded { output } => {
            succeed(id, cx);
            let output = output
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            EpubConversionNotification::Succeeded { output: &output }.push(id, cx);
        }
        JobResult::Failed {
            message,
            issues,
            more_issues,
            staging_path,
            ..
        } => {
            fail(id, message.clone(), cx);
            let staging = staging_path.map(|path| path.display().to_string());
            EpubConversionNotification::Failed {
                name,
                message: &message,
                issues: &issues,
                more_issues,
                staging: staging.as_deref(),
            }
            .push(id, cx);
        }
        JobResult::Cancelled => {
            cancel(id, "cancelled during shutdown".into(), cx);
            EpubConversionNotification::Cancelled.push(id, cx);
        }
        JobResult::Unknown { staging_path } => {
            fail(
                id,
                "the converter stopped before reporting a result".into(),
                cx,
            );
            let staging = staging_path.map(|path| path.display().to_string());
            EpubConversionNotification::Unknown {
                staging: staging.as_deref(),
            }
            .push(id, cx);
        }
    }
}

// Logs carry the outcome and error code only; paths and messages can name the user's files.
fn result_kind(result: &JobResult) -> &str {
    match result {
        JobResult::Succeeded { .. } => "succeeded",
        JobResult::Failed { code, .. } => code,
        JobResult::Cancelled => "cancelled",
        JobResult::Unknown { .. } => "unknown",
    }
}
