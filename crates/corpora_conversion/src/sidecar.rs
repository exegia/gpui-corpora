//! Process lifecycle for the bundled `corpora sidecar --protocol 1`.

use std::{
    fmt,
    io::{self, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::{
    EpubTools,
    protocol::{
        self, MAX_FRAME_BYTES, PROTOCOL_VERSION, PathArgument, Request, SidecarMessage,
        StartOptions,
    },
};

const EPUB_TO_TF: &str = "epub_to_tf";
// Bounded so a stalled consumer applies backpressure to the sidecar instead of buffering.
const EVENT_CAPACITY: usize = 256;
const DROP_GRACE: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// One item from the sidecar's event stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SidecarEvent {
    Message(SidecarMessage),
    /// Standard output closed. Any accepted job without `finished` has an unknown
    /// outcome: keep its staging path and do not retry into the same destination.
    Closed(CloseReason),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CloseReason {
    EndOfStream,
    /// Frame boundaries are lost; the sidecar must be shut down.
    InvalidFrame(String),
    Io(String),
}

impl fmt::Display for CloseReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndOfStream => formatter.write_str("end of stream"),
            Self::InvalidFrame(reason) => write!(formatter, "invalid frame: {reason}"),
            Self::Io(reason) => write!(formatter, "read failed: {reason}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SidecarError {
    #[error("failed to start the EPUB sidecar: {0}")]
    Spawn(#[source] io::Error),
    #[error("the EPUB sidecar did not report ready within {0:?}")]
    ReadyTimeout(Duration),
    #[error("the EPUB sidecar closed before it was ready: {0}")]
    ClosedBeforeReady(CloseReason),
    #[error("the EPUB sidecar sent {0:?} before ready")]
    UnexpectedHandshake(Box<SidecarMessage>),
    #[error("the EPUB sidecar does not offer {0}")]
    Unsupported(&'static str),
    #[error("{} is not an absolute UTF-8 path", .0.display())]
    InvalidPath(PathBuf),
    #[error("failed to send a request to the EPUB sidecar: {0}")]
    Write(#[source] io::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ShutdownOutcome {
    Exited(ExitStatus),
    /// The sidecar ignored end of input for the whole grace period.
    Killed,
}

/// A running sidecar that has completed the `ready` handshake.
///
/// Dropping it closes standard input, which asks the sidecar to cancel active jobs
/// and exit, and kills it if it is still running after a short grace period.
pub struct Sidecar {
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    events: flume::Receiver<SidecarEvent>,
    capabilities: Vec<String>,
    next_request: u64,
}

impl Sidecar {
    /// Start the bundled sidecar and wait for its `ready` message.
    pub fn spawn(tools: &EpubTools, ready_timeout: Duration) -> Result<Self, SidecarError> {
        let mut child = tools
            .sidecar_command()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(SidecarError::Spawn)?;
        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let (sender, events) = flume::bounded(EVENT_CAPACITY);
        if let Some(stdout) = stdout {
            thread::Builder::new()
                .name("epub-sidecar-reader".into())
                .spawn(move || read_frames(stdout, sender))
                .map_err(SidecarError::Spawn)?;
        }

        let mut sidecar = Self {
            child: Some(child),
            stdin,
            events,
            capabilities: Vec::new(),
            next_request: 0,
        };
        sidecar.capabilities = match sidecar.events.recv_timeout(ready_timeout) {
            Ok(SidecarEvent::Message(SidecarMessage::Ready { capabilities })) => capabilities,
            Ok(SidecarEvent::Message(message)) => {
                return Err(SidecarError::UnexpectedHandshake(Box::new(message)));
            }
            Ok(SidecarEvent::Closed(reason)) => {
                return Err(SidecarError::ClosedBeforeReady(reason));
            }
            Err(flume::RecvTimeoutError::Timeout) => {
                return Err(SidecarError::ReadyTimeout(ready_timeout));
            }
            Err(flume::RecvTimeoutError::Disconnected) => {
                return Err(SidecarError::ClosedBeforeReady(CloseReason::EndOfStream));
            }
        };
        Ok(sidecar)
    }

    /// Operations advertised in `ready`.
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Events in the order the sidecar wrote them. The stream ends with one `Closed`.
    ///
    /// Clone the receiver to consume it from another thread or an async task.
    pub fn events(&self) -> &flume::Receiver<SidecarEvent> {
        &self.events
    }

    /// Ask the sidecar to convert `input` into a new Text-Fabric bundle at `output`.
    ///
    /// Returns the request ID; `accepted` or `error` with that ID follows on the
    /// event stream. `output` must not exist yet.
    pub fn start_epub_to_tf(
        &mut self,
        input: &Path,
        output: &Path,
        repair: bool,
    ) -> Result<String, SidecarError> {
        if !self.capabilities.iter().any(|name| name == EPUB_TO_TF) {
            return Err(SidecarError::Unsupported(EPUB_TO_TF));
        }
        let input_path = protocol::protocol_path(input)
            .ok_or_else(|| SidecarError::InvalidPath(input.to_path_buf()))?;
        let output_path = protocol::protocol_path(output)
            .ok_or_else(|| SidecarError::InvalidPath(output.to_path_buf()))?;
        let request_id = self.request_id();
        self.send(&Request::Start {
            protocol: PROTOCOL_VERSION,
            request_id: &request_id,
            operation: EPUB_TO_TF,
            input: PathArgument { path: input_path },
            output: PathArgument { path: output_path },
            options: StartOptions { repair },
        })?;
        Ok(request_id)
    }

    /// Request cancellation of an accepted job and return the request ID.
    ///
    /// A `cancel_ack` only means the request arrived; wait for `finished`.
    pub fn cancel(&mut self, job_id: &str) -> Result<String, SidecarError> {
        let request_id = self.request_id();
        self.send(&Request::Cancel {
            protocol: PROTOCOL_VERSION,
            request_id: &request_id,
            job_id,
        })?;
        Ok(request_id)
    }

    /// Close standard input, then wait up to `grace` before killing the sidecar.
    ///
    /// Blocks the calling thread; run it off the UI thread. Terminal events the
    /// sidecar flushes while exiting remain on cloned event receivers.
    pub fn shutdown(mut self, grace: Duration) -> io::Result<ShutdownOutcome> {
        self.stdin = None;
        match self.child.take() {
            Some(mut child) => wait_or_kill(&mut child, grace),
            None => Err(io::Error::other("sidecar already shut down")),
        }
    }

    fn request_id(&mut self) -> String {
        self.next_request += 1;
        format!("req-{}", self.next_request)
    }

    fn send(&mut self, request: &Request<'_>) -> Result<(), SidecarError> {
        let frame = protocol::encode(request);
        if frame.len() > MAX_FRAME_BYTES {
            return Err(SidecarError::Write(io::Error::new(
                io::ErrorKind::InvalidInput,
                "request exceeds the 1 MiB frame limit",
            )));
        }
        let stdin = self.stdin.as_mut().ok_or_else(|| {
            SidecarError::Write(io::Error::new(io::ErrorKind::BrokenPipe, "stdin closed"))
        })?;
        stdin
            .write_all(&frame)
            .and_then(|()| stdin.flush())
            .map_err(SidecarError::Write)
    }
}

impl Drop for Sidecar {
    fn drop(&mut self) {
        self.stdin = None;
        if let Some(mut child) = self.child.take() {
            // Without a reaper thread the child still sees end of input and exits on its own.
            let _ = thread::Builder::new()
                .name("epub-sidecar-reaper".into())
                .spawn(move || {
                    let _ = wait_or_kill(&mut child, DROP_GRACE);
                });
        }
    }
}

fn wait_or_kill(child: &mut Child, grace: Duration) -> io::Result<ShutdownOutcome> {
    let deadline = Instant::now() + grace;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(ShutdownOutcome::Exited(status));
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            return Ok(ShutdownOutcome::Killed);
        }
        thread::sleep(POLL_INTERVAL);
    }
}

fn read_frames(stdout: ChildStdout, sender: flume::Sender<SidecarEvent>) {
    let mut reader = BufReader::new(stdout);
    let mut frame = Vec::new();
    let reason = loop {
        frame.clear();
        let limit = u64::try_from(MAX_FRAME_BYTES).unwrap_or(u64::MAX);
        match (&mut reader).take(limit).read_until(b'\n', &mut frame) {
            Ok(0) => break CloseReason::EndOfStream,
            Ok(_) if frame.last() != Some(&b'\n') => {
                break CloseReason::InvalidFrame(if frame.len() >= MAX_FRAME_BYTES {
                    "frame exceeds 1 MiB".into()
                } else {
                    "unterminated frame".into()
                });
            }
            Ok(_) => match protocol::decode(&frame) {
                Ok(message) => {
                    if sender.send(SidecarEvent::Message(message)).is_err() {
                        return;
                    }
                }
                Err(reason) => break CloseReason::InvalidFrame(reason),
            },
            Err(error) => break CloseReason::Io(error.to_string()),
        }
    };
    let _ = sender.send(SidecarEvent::Closed(reason));
}

#[cfg(test)]
#[path = "sidecar.test.rs"]
mod tests;
