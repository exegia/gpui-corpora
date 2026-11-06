#[allow(unused_imports)]
use super::*;

pub(crate) fn test_button(
    id: &'static str,
    title_key: &'static str,
    request: impl Fn(String) -> NotificationRequest + 'static,
) -> Button {
    Button::new(id, crate::i18n::localize(title_key))
        .variant(ButtonVariant::Outline)
        .on_click(move |_, _, cx| {
            notifications::send(request(crate::i18n::localize(title_key)), cx);
        })
}
