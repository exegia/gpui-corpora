#[allow(unused_imports)]
use super::*;

pub(crate) fn ensure_shortcut_at(programs: &Path, app_id: &str, exe: &Path) -> Result<(), String> {
    let shortcut = programs.join(SHORTCUT_NAME);
    with_com(|| {
        if shortcut_matches(&shortcut, app_id, exe)? {
            return Ok(());
        }
        write_shortcut(&shortcut, app_id, exe)
    })
}
