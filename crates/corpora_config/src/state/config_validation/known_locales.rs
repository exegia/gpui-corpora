#[allow(unused_imports)]
use super::*;

/// Locales the app ships translations for. Mirrors the two constants the rest
/// of the crate treats as authoritative (`LOCALE_EN`, `LOCALE_ZH_CN`).
pub(crate) const KNOWN_LOCALES: &[&str] = &[LOCALE_EN, LOCALE_ZH_CN];
