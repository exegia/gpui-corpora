#[allow(unused_imports)]
use super::*;

/// Shortcut lives in the user's Start Menu Programs folder (no admin rights).
pub(crate) fn programs_dir() -> Result<PathBuf, String> {
    unsafe {
        let pwstr = SHGetKnownFolderPath(&FOLDERID_Programs, KNOWN_FOLDER_FLAG(0), None)
            .map_err(|err| format!("SHGetKnownFolderPath(Programs): {err}"))?;
        let decoded = pwstr_to_string(pwstr);
        CoTaskMemFree(Some(pwstr.as_ptr().cast()));
        decoded.map(PathBuf::from)
    }
}
