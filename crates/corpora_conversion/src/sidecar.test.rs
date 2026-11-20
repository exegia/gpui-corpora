#![cfg(unix)]

use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use serde_json::json;

use super::{CloseReason, ShutdownOutcome, Sidecar, SidecarError, SidecarEvent};
use crate::{
    EpubTools,
    protocol::{
        DiagnosticLocation, DiagnosticSeverity, JobError, JobOutcome, JobOutput, OutputKind,
        ProgressPhase, SidecarMessage, decode,
    },
};

const READY: &str =
    r#"printf '%s\n' '{"protocol":1,"type":"ready","capabilities":["epub_to_tf"]}'"#;
const TIMEOUT: Duration = Duration::from_secs(10);

// Scripts `exec` long-running commands: on macOS a concurrently spawned child can inherit
// another test's pipe, and killing `sh` would leave that grandchild holding it open.
fn fake_sidecar(directory: &Path, script: &str) -> EpubTools {
    let entry = directory.join("sidecar.sh");
    fs::write(&entry, script).expect("write fake sidecar");
    EpubTools {
        checker: directory.join("corpora-epub-check"),
        exporter: directory.join("epub-to-tf"),
        python: PathBuf::from("/bin/sh"),
        sidecar_entry: entry,
    }
}

fn spawn(tools: &EpubTools) -> Sidecar {
    Sidecar::spawn(tools, TIMEOUT).expect("sidecar handshake")
}

fn next_event(sidecar: &Sidecar) -> SidecarEvent {
    sidecar
        .events()
        .recv_timeout(TIMEOUT)
        .expect("sidecar event")
}

fn message(event: SidecarEvent) -> SidecarMessage {
    match event {
        SidecarEvent::Message(message) => message,
        SidecarEvent::Closed(reason) => panic!("stream closed early: {reason}"),
    }
}

#[test]
fn runs_a_job_through_the_protocol() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let request_log = directory.path().join("request.json");
    let tools = fake_sidecar(
        directory.path(),
        &format!(
            r#"{READY}
IFS= read -r request
printf '%s\n' "$request" > '{log}'
printf '%s\n' '{{"protocol":1,"type":"accepted","request_id":"req-1","job_id":"job-1","staging_path":"/books/.out-job-1"}}'
printf '%s\n' '{{"protocol":1,"type":"progress","job_id":"job-1","phase":"validating","message":"Checking the EPUB"}}'
printf '%s\n' '{{"protocol":1,"type":"diagnostic","job_id":"job-1","severity":"warning","code":"EXAMPLE","message":"Review","location":{{"member":"chapter.xhtml","line":12}}}}'
printf '%s\n' '{{"protocol":1,"type":"finished","job_id":"job-1","outcome":"succeeded","outputs":[{{"kind":"tf_bundle","path":"/books/out"}}]}}'
exec cat > /dev/null
"#,
            log = request_log.display()
        ),
    );
    let mut sidecar = spawn(&tools);
    assert_eq!(sidecar.capabilities(), ["epub_to_tf"]);

    let request_id = sidecar
        .start_epub_to_tf(
            Path::new("/books/book.epub"),
            Path::new("/books/out"),
            false,
        )
        .expect("send start");

    assert_eq!(request_id, "req-1");
    assert_eq!(
        message(next_event(&sidecar)),
        SidecarMessage::Accepted {
            request_id: "req-1".into(),
            job_id: "job-1".into(),
            staging_path: "/books/.out-job-1".into(),
        }
    );
    assert_eq!(
        message(next_event(&sidecar)),
        SidecarMessage::Progress {
            job_id: "job-1".into(),
            phase: ProgressPhase::Validating,
            message: "Checking the EPUB".into(),
        }
    );
    assert_eq!(
        message(next_event(&sidecar)),
        SidecarMessage::Diagnostic {
            job_id: "job-1".into(),
            severity: DiagnosticSeverity::Warning,
            code: "EXAMPLE".into(),
            message: "Review".into(),
            location: Some(DiagnosticLocation {
                member: Some("chapter.xhtml".into()),
                line: Some(12),
                column: None,
            }),
        }
    );
    assert_eq!(
        message(next_event(&sidecar)),
        SidecarMessage::Finished {
            job_id: "job-1".into(),
            outcome: JobOutcome::Succeeded,
            outputs: vec![JobOutput {
                kind: OutputKind::TfBundle,
                path: "/books/out".into(),
            }],
            error: None,
        }
    );

    let events = sidecar.events().clone();
    let outcome = sidecar.shutdown(TIMEOUT).expect("shut down sidecar");
    assert!(matches!(outcome, ShutdownOutcome::Exited(status) if status.success()));
    assert_eq!(
        events.recv_timeout(TIMEOUT).expect("close event"),
        SidecarEvent::Closed(CloseReason::EndOfStream)
    );

    let request: serde_json::Value =
        serde_json::from_slice(&fs::read(&request_log).expect("read request log"))
            .expect("parse start request");
    assert_eq!(
        request,
        json!({
            "type": "start",
            "protocol": 1,
            "request_id": "req-1",
            "operation": "epub_to_tf",
            "input": {"path": "/books/book.epub"},
            "output": {"path": "/books/out"},
            "options": {"repair": false},
        })
    );
}

#[test]
fn reports_an_unknown_outcome_when_the_sidecar_exits_mid_job() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        &format!(
            r#"{READY}
IFS= read -r request
printf '%s\n' '{{"protocol":1,"type":"accepted","request_id":"req-1","job_id":"job-1","staging_path":"/books/.out-job-1"}}'
exit 1
"#
        ),
    );
    let mut sidecar = spawn(&tools);
    sidecar
        .start_epub_to_tf(
            Path::new("/books/book.epub"),
            Path::new("/books/out"),
            false,
        )
        .expect("send start");

    assert!(matches!(
        message(next_event(&sidecar)),
        SidecarMessage::Accepted { .. }
    ));
    assert_eq!(
        next_event(&sidecar),
        SidecarEvent::Closed(CloseReason::EndOfStream)
    );
}

#[test]
fn rejects_relative_paths_before_sending() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        &format!("{READY}\nexec cat > /dev/null\n"),
    );
    let mut sidecar = spawn(&tools);

    let error = sidecar
        .start_epub_to_tf(Path::new("book.epub"), Path::new("/books/out"), false)
        .expect_err("relative input");

    assert!(matches!(error, SidecarError::InvalidPath(path) if path == Path::new("book.epub")));
}

#[test]
fn refuses_to_start_without_the_capability() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        r#"printf '%s\n' '{"protocol":1,"type":"ready","capabilities":[]}'
exec cat > /dev/null
"#,
    );
    let mut sidecar = spawn(&tools);

    let error = sidecar
        .start_epub_to_tf(
            Path::new("/books/book.epub"),
            Path::new("/books/out"),
            false,
        )
        .expect_err("no capability");

    assert!(matches!(error, SidecarError::Unsupported("epub_to_tf")));
}

#[test]
fn closes_the_stream_on_an_unknown_field() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        &format!(
            r#"{READY}
printf '%s\n' '{{"protocol":1,"type":"progress","job_id":"job-1","phase":"validating","message":"m","percent":5}}'
exec cat > /dev/null
"#
        ),
    );
    let sidecar = spawn(&tools);

    assert!(matches!(
        next_event(&sidecar),
        SidecarEvent::Closed(CloseReason::InvalidFrame(_))
    ));
}

#[test]
fn closes_the_stream_on_an_oversized_frame() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        &format!("{READY}\nhead -c 1100000 /dev/zero | tr '\\0' 'a'\nprintf '\\n'\n"),
    );
    let sidecar = spawn(&tools);

    assert_eq!(
        next_event(&sidecar),
        SidecarEvent::Closed(CloseReason::InvalidFrame("frame exceeds 1 MiB".into()))
    );
}

#[test]
fn fails_the_handshake_when_ready_is_not_first() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(
        directory.path(),
        r#"printf '%s\n' '{"protocol":1,"type":"protocol_error","code":"incompatible_protocol","message":"no"}'
"#,
    );

    let error = Sidecar::spawn(&tools, TIMEOUT)
        .err()
        .expect("handshake fails");

    assert!(matches!(error, SidecarError::UnexpectedHandshake(_)));
}

#[test]
fn times_out_when_ready_never_arrives() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(directory.path(), "exec sleep 30\n");

    let error = Sidecar::spawn(&tools, Duration::from_millis(200))
        .err()
        .expect("handshake times out");

    assert!(matches!(error, SidecarError::ReadyTimeout(_)));
}

#[test]
fn kills_a_sidecar_that_ignores_end_of_input() {
    let directory = tempfile::tempdir().expect("create fixture directory");
    let tools = fake_sidecar(directory.path(), &format!("{READY}\nexec sleep 30\n"));
    let sidecar = spawn(&tools);

    let outcome = sidecar
        .shutdown(Duration::from_millis(200))
        .expect("shut down sidecar");

    assert_eq!(outcome, ShutdownOutcome::Killed);
}

#[test]
fn decodes_a_cleanup_failure_with_its_staging_path() {
    let frame = br#"{"protocol":1,"type":"finished","job_id":"job-1","outcome":"failed","outputs":[],"error":{"code":"cleanup_incomplete","message":"Cleanup failed","staging_path":"/books/.out-job-1"}}"#;

    assert_eq!(
        decode(frame),
        Ok(SidecarMessage::Finished {
            job_id: "job-1".into(),
            outcome: JobOutcome::Failed,
            outputs: Vec::new(),
            error: Some(JobError {
                code: "cleanup_incomplete".into(),
                message: "Cleanup failed".into(),
                staging_path: Some("/books/.out-job-1".into()),
            }),
        })
    );
}

#[test]
fn rejects_other_protocol_versions_and_unknown_types() {
    assert!(decode(br#"{"protocol":2,"type":"ready","capabilities":[]}"#).is_err());
    assert!(decode(br#"{"type":"ready","capabilities":[]}"#).is_err());
    assert!(decode(br#"{"protocol":1,"type":"heartbeat"}"#).is_err());
}
