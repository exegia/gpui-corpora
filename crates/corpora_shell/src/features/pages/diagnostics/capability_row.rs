#[allow(unused_imports)]
use super::*;

pub(crate) fn capability_row(name: &str, value: &str) -> (String, String) {
    (
        format!("{}:{name}", crate::i18n::localize("diagnostics_capability")),
        value.to_string(),
    )
}
