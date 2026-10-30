#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct SecureStorageSnapshot {
    pub available: bool,
    pub last_error: Option<String>,
}

impl Global for SecureStorageSnapshot {}
