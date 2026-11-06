#[allow(unused_imports)]
use super::*;

// i18n.test.rs asserts this sentinel renders as the key itself, so it must
// stay out of the catalogs.
pub(crate) const MISSING_BY_DESIGN: &[&str] = &["i18n_no_such_key"];
