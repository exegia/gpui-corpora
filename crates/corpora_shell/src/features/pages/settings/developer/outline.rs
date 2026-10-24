#[allow(unused_imports)]
use super::*;

pub(crate) fn outline(id: &'static str, key: &str) -> Button {
    Button::new(id, crate::i18n::localize(key)).variant(ButtonVariant::Outline)
}
