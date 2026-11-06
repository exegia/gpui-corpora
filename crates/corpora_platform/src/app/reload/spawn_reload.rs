#[allow(unused_imports)]
use super::*;

/// Spawn the replacement with the original argv, then return so the caller
/// can exit; the child retries the instance lock while this process dies.
#[cfg(windows)]
pub(crate) fn spawn_reload() -> Result<()> {
    let exe = std::env::current_exe().context("failed to resolve current executable path")?;
    let mut cmd = std::process::Command::new(&exe);
    // Skip argv[0] (the program name) since Command::new already supplies
    // argv[0] from `exe`; args_os round-trips non-UTF8 argv where args() panics.
    cmd.args(std::env::args_os().skip(1));
    cmd.env(RESTART_HANDOFF_ENV, "1");

    let child = cmd.spawn().map_err(|err| {
        error!(target: LOG, error = %err, "reload spawn failed");
        err
    })?;
    info!(target: LOG, pid = child.id(), "spawned replacement process");
    drop(child); // releasing the handle does not kill the child
    Ok(())
}
