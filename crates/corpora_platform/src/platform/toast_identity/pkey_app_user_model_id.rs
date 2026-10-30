#[allow(unused_imports)]
use super::*;

// `System.AppUserModel_ID`; the windows crate ships it only behind the
// oversized Win32_Storage_EnhancedStorage feature.
pub(crate) const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
    fmtid: GUID::from_u128(0x9f4c2855_9f79_4b39_a8d0_e1d42de1d5f3),
    pid: 5,
};
