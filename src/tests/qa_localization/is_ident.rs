#[allow(unused_imports)]
use super::*;

pub(crate) fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
