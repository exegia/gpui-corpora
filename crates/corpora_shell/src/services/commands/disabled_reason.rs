#[allow(unused_imports)]
use super::*;

pub(crate) fn disabled_reason(key: &str, disabled: bool) -> Option<SharedString> {
    disabled.then(|| crate::i18n::localize(key).into())
}
