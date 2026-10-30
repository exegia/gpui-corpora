#[allow(unused_imports)]
use super::*;

pub(crate) fn write_shortcut(
    shortcut: &Path,
    app_id: &str,
    exe: &Path,
) -> Result<(), windows::core::Error> {
    let link: IShellLinkW = unsafe { CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_ALL)? };
    let exe_path = HSTRING::from(exe.as_os_str());
    unsafe {
        link.SetPath(&exe_path)?;
        link.SetDescription(&HSTRING::from("Corpora"))?;
        if let Some(dir) = exe.parent() {
            link.SetWorkingDirectory(&HSTRING::from(dir.as_os_str()))?;
        }
        link.SetIconLocation(&exe_path, 0)?;
    }

    let store: IPropertyStore = link.cast()?;
    let mut propvar = propvariant_from_str(app_id)?;
    unsafe {
        store.SetValue(&PKEY_APP_USER_MODEL_ID, &propvar)?;
        PropVariantClear(&mut propvar)?;
    }

    let persist: IPersistFile = link.cast()?;
    unsafe { persist.Save(&HSTRING::from(shortcut.as_os_str()), true)? };
    tracing::info!(
        target: LOG,
        shortcut = %shortcut.display(),
        "start-menu shortcut with toast AUMID written"
    );
    Ok(())
}
