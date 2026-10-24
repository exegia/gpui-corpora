#[allow(unused_imports)]
use super::*;

pub(crate) fn pwstr_to_string(pwstr: PWSTR) -> Result<String, String> {
    let mut len = 0usize;
    unsafe {
        while *pwstr.0.add(len) != 0 {
            len += 1;
        }
        String::from_utf16(std::slice::from_raw_parts(pwstr.0, len))
            .map_err(|err| format!("non-UTF16 path: {err}"))
    }
}
