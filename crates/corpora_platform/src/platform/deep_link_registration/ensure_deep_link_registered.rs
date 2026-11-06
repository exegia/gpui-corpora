#[allow(unused_imports)]
use super::*;

/// Register the `corpora://` scheme for the current exe under HKCU (no
/// admin rights): the `URL Protocol` marker plus a `shell\open\command`
/// pointing at the running exe. `Err(reason)` never blocks startup — callers
/// log and continue.
pub fn ensure_deep_link_registered() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|err| format!("current_exe: {err}"))?;
    let command = format!("\"{}\" \"%1\"", exe.display());

    let scheme = windows_registry::CURRENT_USER
        .create(format!("Software\\Classes\\{}", scheme_key()))
        .map_err(|err| format!("create scheme key: {err}"))?;
    // RegCreateKeyExW creates one level per call, so walk down to the command key.
    let command_key = scheme
        .create("shell")
        .and_then(|shell| shell.create("open"))
        .and_then(|open| open.create("command"))
        .map_err(|err| format!("create command key: {err}"))?;

    // Self-healing fast path: a stored command already pointing at this exe is
    // left untouched; anything else (first run, exe moved between dev target
    // dirs, garbage) is rewritten.
    if let Ok(existing) = command_key.get_string("")
        && command_matches(&existing, &exe)
    {
        tracing::debug!(target: LOG, "deep-link registration already current");
        return Ok(());
    }

    // An empty value name is the key default; `URL Protocol` is what marks the
    // key a protocol handler.
    scheme
        .set_string("", "URL:Corpora")
        .and_then(|()| scheme.set_string("URL Protocol", ""))
        .and_then(|()| command_key.set_string("", &command))
        .map_err(|err| format!("write scheme values: {err}"))?;
    tracing::info!(target: LOG, command = %command, "deep-link scheme registered for the current exe");
    Ok(())
}
