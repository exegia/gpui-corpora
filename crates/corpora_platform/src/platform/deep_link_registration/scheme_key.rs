#[allow(unused_imports)]
use super::*;

/// Registry key name for the scheme: `corpora://` minus `://`. Derived,
/// never a second copy of the scheme string.
pub(crate) fn scheme_key() -> String {
    SCHEME.strip_suffix("://").unwrap_or(SCHEME).to_string()
}
