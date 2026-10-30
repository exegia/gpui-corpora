#[allow(unused_imports)]
use super::*;

// IShellLink coclass; the windows crate does not generate this CLSID const.
pub(crate) const CLSID_SHELL_LINK: GUID = GUID::from_u128(0x00021401_0000_0000_c000_000000000046);
