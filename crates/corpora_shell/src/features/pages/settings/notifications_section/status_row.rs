#[allow(unused_imports)]
use super::*;

pub(crate) fn status_row(label: &str, value: impl Into<SharedString>) -> SettingsRow {
    SettingsRow::new(crate::i18n::localize(label)).description(value)
}
