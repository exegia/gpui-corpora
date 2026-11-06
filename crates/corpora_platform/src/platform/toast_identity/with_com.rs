#[allow(unused_imports)]
use super::*;

pub(crate) fn with_com(
    work: impl FnOnce() -> Result<(), windows::core::Error>,
) -> Result<(), String> {
    let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    // S_FALSE = already initialized (usable, still must balance);
    // RPC_E_CHANGED_MODE = foreign apartment (usable, not ours to tear down).
    let balance = hr.is_ok();
    if !balance && hr != RPC_E_CHANGED_MODE {
        return Err(format!("CoInitializeEx: {hr}"));
    }
    let result = work();
    if balance {
        unsafe { CoUninitialize() };
    }
    result.map_err(|err| err.to_string())
}
