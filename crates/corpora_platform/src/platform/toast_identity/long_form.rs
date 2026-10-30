#[allow(unused_imports)]
use super::*;

/// GetLongPathNameW returns 0 for a path it cannot expand and the needed
/// size (null included) when the buffer is too small; both mean "no".
pub(crate) fn long_form(path: &str) -> Option<String> {
    let wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let mut buffer = [0u16; 1024];
    let len = unsafe { GetLongPathNameW(PCWSTR::from_raw(wide.as_ptr()), Some(&mut buffer)) };
    if len == 0 || len as usize >= buffer.len() {
        return None;
    }
    String::from_utf16(&buffer[..len as usize]).ok()
}
