#[allow(unused_imports)]
use super::*;

pub(crate) fn propvariant_string(propvar: &PROPVARIANT) -> Option<String> {
    unsafe {
        let inner = &*propvar.Anonymous.Anonymous;
        if inner.vt != VT_LPWSTR {
            return None;
        }
        pwstr_to_string(inner.Anonymous.pwszVal).ok()
    }
}
