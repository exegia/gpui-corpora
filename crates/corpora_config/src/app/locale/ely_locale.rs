#[allow(unused_imports)]
use super::*;

/// Ely's locale tag for an app locale; Ely components read their builtin copy in it.
pub(crate) fn ely_locale(locale: &str) -> &'static str {
    if locale == LOCALE_ZH_CN {
        "zh-CN"
    } else {
        "en-US"
    }
}
