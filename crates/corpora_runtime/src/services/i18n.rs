#[cfg(test)]
#[path = "i18n.test/mod.rs"]
mod i18n_test;

mod detect_system_locale;
mod localize;
mod localize_with_args;

pub use detect_system_locale::detect_system_locale;
pub use localize::localize;
pub use localize_with_args::localize_with_args;
