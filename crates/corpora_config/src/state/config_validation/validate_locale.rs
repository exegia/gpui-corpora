#[allow(unused_imports)]
use super::*;

pub(crate) fn validate_locale(locale: &str) -> Vec<ConfigLint> {
    if locale.is_empty() {
        return vec![ConfigLint::new(
            "locale",
            LintSeverity::Warning,
            format!("locale is empty; falling back to {LOCALE_EN:?}"),
        )];
    }
    if !KNOWN_LOCALES.contains(&locale) {
        return vec![ConfigLint::new(
            "locale",
            LintSeverity::Warning,
            format!(
                "locale = {locale:?} has no bundled translations (expected one of {:?}); \
                 falling back to {LOCALE_EN:?}",
                KNOWN_LOCALES
            ),
        )];
    }
    Vec::new()
}
