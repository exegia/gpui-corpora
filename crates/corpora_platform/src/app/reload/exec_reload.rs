#[allow(unused_imports)]
use super::*;

/// Re-exec with the original argv after GPUI shutdown; never returns on
/// success, returns `Err` without panicking otherwise.
#[cfg(unix)]
pub(crate) fn exec_reload() -> Result<()> {
    use std::os::unix::process::CommandExt;

    info!(target: LOG, "executing in-place reload");

    let exe = std::env::current_exe().context("failed to resolve current executable path")?;

    // Skip argv[0] (the program name) since Command::new already supplies
    // argv[0] from `exe`.
    let mut cmd = std::process::Command::new(&exe);
    cmd.args(std::env::args().skip(1));

    let err = cmd.exec();

    // exec() returns the io::Error only when it fails — success never returns.
    error!(target: LOG, error = %err, "exec failed");
    Err(anyhow::anyhow!("failed to exec reload: {err}"))
}
