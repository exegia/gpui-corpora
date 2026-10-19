#[allow(unused_imports)]
use super::*;

pub(crate) fn row(label_key: &str, value: &str) -> (String, String) {
    (crate::i18n::localize(label_key), value.to_string())
}
