#[allow(unused_imports)]
use super::*;

pub fn set_locale(locale: &str, cx: &mut App) {
    rust_i18n::set_locale(locale);
    if cx.has_global::<I18n>() {
        I18n::set_locale(ely_locale(locale), cx);
    }
    cx.set_global::<LocaleState>(LocaleState(SharedString::from(locale.to_string())));
    crate::app_state::update_config(cx, |config| {
        config.locale = locale.to_string();
    });
    cx.refresh_windows();
}
