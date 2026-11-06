#[allow(unused_imports)]
use super::*;

/// Relaunch after GPUI shutdown: exec in place on Unix, spawn-and-return on
/// Windows.
pub fn perform_reload() -> Result<()> {
    #[cfg(unix)]
    {
        exec_reload()
    }
    #[cfg(windows)]
    {
        spawn_reload()
    }
}
