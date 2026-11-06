#[allow(unused_imports)]
use super::*;

pub(crate) fn has_lint_for(lints: &[ConfigLint], field: &str) -> bool {
    lints.iter().any(|l| l.field == field)
}
