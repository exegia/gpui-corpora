#[allow(unused_imports)]
use super::*;

/// Set while a debounce timer is armed, so concurrent mutations reuse the
/// pending timer instead of spawning another one.
pub(crate) static SAVE_SCHEDULED: AtomicBool = AtomicBool::new(false);
