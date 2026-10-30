#[allow(unused_imports)]
use super::*;

/// Lint ceiling for one window dimension; the store's corruption ceiling
/// (`MAX_PLAUSIBLE_DIM` in config_store) is what drops bounds outright.
pub(crate) const MAX_WINDOW_DIM: f32 = 8192.0;
