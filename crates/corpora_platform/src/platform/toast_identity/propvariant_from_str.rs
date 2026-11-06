#[allow(unused_imports)]
use super::*;

/// PROPVARIANT::VT_LPWSTR pointing at a CoTaskMemAlloc'd copy of `text`, so
/// `PropVariantClear` frees it correctly.
pub(crate) fn propvariant_from_str(text: &str) -> Result<PROPVARIANT, windows::core::Error> {
    let wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    let bytes = wide.len() * std::mem::size_of::<u16>();
    let buffer = unsafe { CoTaskMemAlloc(bytes) };
    if buffer.is_null() {
        return Err(windows::core::Error::from_hresult(E_OUTOFMEMORY));
    }
    unsafe {
        std::ptr::copy_nonoverlapping(wide.as_ptr().cast::<u8>(), buffer.cast::<u8>(), bytes);
        let mut propvar = PROPVARIANT::default();
        let inner = &mut *propvar.Anonymous.Anonymous;
        inner.vt = VT_LPWSTR;
        inner.Anonymous.pwszVal = PWSTR(buffer.cast());
        Ok(propvar)
    }
}
