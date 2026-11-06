#[allow(unused_imports)]
use super::*;

/// A missing, unreadable, or stale shortcut is simply rewritten.
pub(crate) fn shortcut_matches(
    shortcut: &Path,
    app_id: &str,
    exe: &Path,
) -> Result<bool, windows::core::Error> {
    let Ok(link) = load_shortcut(shortcut) else {
        return Ok(false);
    };

    let mut buffer = [0u16; 1024];
    let mut find_data = WIN32_FIND_DATAW::default();
    unsafe { link.GetPath(&mut buffer, &mut find_data, 0)? };
    let target = pwstr_to_string(PWSTR(buffer.as_mut_ptr()))
        .map(PathBuf::from)
        .unwrap_or_default();
    if !same_path(&target, exe) {
        return Ok(false);
    }

    let store: IPropertyStore = link.cast()?;
    let mut propvar = unsafe { store.GetValue(&PKEY_APP_USER_MODEL_ID)? };
    let aumid = propvariant_string(&propvar);
    unsafe { PropVariantClear(&mut propvar)? };
    Ok(aumid.as_deref() == Some(app_id))
}
