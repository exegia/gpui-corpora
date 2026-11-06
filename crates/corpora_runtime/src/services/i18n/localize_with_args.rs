#[allow(unused_imports)]
use super::*;

/// [`localize`] with `%{name}` placeholder substitution, applying the same
/// replacement `rust_i18n::t!` uses for inline arguments.
pub fn localize_with_args(id: &str, args: &[(&str, &str)]) -> String {
    let patterns: Vec<&str> = args.iter().map(|(name, _)| *name).collect();
    let values: Vec<String> = args.iter().map(|(_, value)| value.to_string()).collect();
    rust_i18n::replace_patterns(&rust_i18n::t!(id), &patterns, &values)
}
