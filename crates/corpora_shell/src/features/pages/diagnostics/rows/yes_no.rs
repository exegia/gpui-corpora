#[allow(unused_imports)]
use super::*;

pub(crate) fn yes_no(value: bool) -> String {
    crate::i18n::localize(if value {
        "diagnostics_yes"
    } else {
        "diagnostics_no"
    })
}
