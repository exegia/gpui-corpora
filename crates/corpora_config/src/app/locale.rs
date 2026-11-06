use corpora_ui::ely::i18n::I18n;
use gpui::{App, Global, SharedString};

mod current_locale;
mod ely_locale;
mod install_ely_i18n;
mod locale_en;
mod locale_state;
mod locale_zh_cn;
mod set_locale;

pub use current_locale::current_locale;
use ely_locale::ely_locale;
pub use install_ely_i18n::install_ely_i18n;
pub use locale_en::LOCALE_EN;
pub use locale_state::LocaleState;
pub use locale_zh_cn::LOCALE_ZH_CN;
pub use set_locale::set_locale;
