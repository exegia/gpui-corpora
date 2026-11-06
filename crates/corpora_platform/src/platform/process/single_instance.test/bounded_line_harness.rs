#[allow(unused_imports)]
use super::*;

/// Listener plus a connected client on a throwaway name; the reader runs on
/// a thread because `read_bounded_line` blocks until a frame terminates.
pub(crate) fn bounded_line_harness(payload: usize) -> Option<String> {
    let unique_name = format!("com.corpora.tests.{}", uuid::Uuid::new_v4());
    let name = resolve_ipc_name(&unique_name).expect("resolve name");
    let listener = ListenerOptions::new()
        .name(name)
        .create_sync()
        .expect("create listener");
    let (tx, rx) = mpsc::channel::<Option<String>>();
    std::thread::spawn(move || {
        let conn = listener.incoming().next().and_then(|c| c.ok());
        let _ = tx.send(conn.as_ref().and_then(read_bounded_line));
    });

    let mut client = interprocess::local_socket::Stream::connect(
        resolve_ipc_name(&unique_name).expect("resolve client name"),
    )
    .expect("connect client");
    let write = writeln!(client, "{}", "a".repeat(payload));
    // Past the cap the reader closes after MAX+1 bytes, so the write tail
    // surfaces as BrokenPipe on Windows named pipes — still a dropped frame.
    if payload > MAX_LINE_BYTES {
        if let Err(error) = write {
            assert_eq!(error.kind(), ErrorKind::BrokenPipe);
        }
    } else {
        write.expect("write frame");
    }
    rx.recv_timeout(Duration::from_secs(3))
        .expect("reader finished")
}
