#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct ShortcutState {
    pub enabled: bool,
    pub registered: bool,
    pub accelerator: String,
    pub last_error: Option<String>,
}

impl Global for ShortcutState {}
