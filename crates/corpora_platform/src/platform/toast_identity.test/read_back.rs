#[allow(unused_imports)]
use super::*;

/// Decode the shortcut again through the shell APIs, independent of the
/// shortcut_matches fast path, so the assertions observe the persisted file.
pub(crate) fn read_back(programs: &std::path::Path) -> (PathBuf, Option<String>) {
    use windows::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
    use windows::Win32::System::Com::StructuredStorage::PropVariantClear;
    use windows::Win32::System::Com::{
        CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
        IPersistFile, STGM_READ,
    };
    use windows::Win32::UI::Shell::IShellLinkW;
    use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
    use windows::core::{HSTRING, Interface};

    let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    assert!(hr.is_ok(), "COM init in test: {hr}");

    let result = (|| {
        let link: IShellLinkW =
            unsafe { CoCreateInstance(&super::super::CLSID_SHELL_LINK, None, CLSCTX_ALL) }
                .expect("shell link instance");
        let persist: IPersistFile = link.cast().expect("IPersistFile");
        let lnk = programs.join("corpora.lnk");
        unsafe {
            persist
                .Load(&HSTRING::from(lnk.as_os_str()), STGM_READ)
                .expect("load lnk")
        };

        let mut buffer = [0u16; 1024];
        let mut find_data = WIN32_FIND_DATAW::default();
        unsafe {
            link.GetPath(&mut buffer, &mut find_data, 0)
                .expect("read target")
        };
        let target =
            String::from_utf16_lossy(&buffer[..buffer.iter().position(|&c| c == 0).unwrap()]);

        let store: IPropertyStore = link.cast().expect("IPropertyStore");
        let mut propvar = unsafe {
            store
                .GetValue(&super::super::PKEY_APP_USER_MODEL_ID)
                .expect("read AUMID")
        };
        let aumid = super::super::propvariant_string(&propvar);
        unsafe { PropVariantClear(&mut propvar).expect("clear propvariant") };
        (PathBuf::from(target), aumid)
    })();

    unsafe { CoUninitialize() };
    result
}
