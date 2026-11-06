#[allow(unused_imports)]
use super::*;

// SAVE_SCHEDULED is process-global while each test gets its own App, so
// debounce-arming tests serialize on this poison-tolerant lock.
pub(crate) static DEBOUNCE_TESTS: Mutex<()> = Mutex::new(());
