#[allow(unused_imports)]
use super::*;

/// Upper bound for the state file. A larger file is corrupt by definition, and
/// parsing it would allocate unbounded memory from untrusted input at startup.
pub(crate) const MAX_STATE_FILE_BYTES: u64 = 1024 * 1024;
