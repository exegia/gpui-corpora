#[allow(unused_imports)]
use super::*;

pub(crate) fn load_shortcut(shortcut: &Path) -> Result<IShellLinkW, windows::core::Error> {
    let link: IShellLinkW = unsafe { CoCreateInstance(&CLSID_SHELL_LINK, None, CLSCTX_ALL)? };
    let persist: IPersistFile = link.cast()?;
    unsafe { persist.Load(&HSTRING::from(shortcut.as_os_str()), STGM_READ)? };
    Ok(link)
}
