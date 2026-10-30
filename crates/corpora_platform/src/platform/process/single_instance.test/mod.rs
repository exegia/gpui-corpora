use std::{
    io::{BufRead, ErrorKind, Write},
    sync::mpsc,
    time::Duration,
};

use tempfile::tempdir;

use super::{
    AcquireOutcome, MAX_LINE_BYTES, SCHEME, append_forwarded_link, drain_forwarded_links,
    read_bounded_line, resolve_ipc_name, retry_until_single, send_forwarded_link_via_ipc,
};
use interprocess::local_socket::{GenericNamespaced, ListenerOptions, prelude::*};

// === split-modules: generated submodules ===
mod bounded_line_harness;
mod forwarded_links_roundtrip_in_order;
mod forwards_link_over_ipc;
mod ipc_frame_at_bound_is_accepted;
mod oversized_ipc_frame_is_dropped;
mod retry_acquires_after_busy_attempts;
mod retry_acquires_on_first_attempt_without_sleeping;
mod retry_exhausts_attempts_into_busy;
mod retry_failed_short_circuits_without_sleeping;

use bounded_line_harness::bounded_line_harness;
