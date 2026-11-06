#[allow(unused_imports)]
use super::*;

/// Ely's `I18n` global with empty app catalogs, so components fall back to
/// Ely's builtin copy for the shown locale.
pub fn install_ely_i18n(cx: &mut App) {
    cx.set_global(
        I18n::new("en-US")
            .catalog("en-US", &[])
            .catalog("zh-CN", &[]),
    );
}
