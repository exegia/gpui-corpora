#[allow(unused_imports)]
use super::*;

// Caps keep reports (and the upload payload) bounded; scrubbing keeps the
// user's home path out of anything that leaves the machine.
pub(crate) const MAX_PANIC_MESSAGE_CHARS: usize = 2048;
