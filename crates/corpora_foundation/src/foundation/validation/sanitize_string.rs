#[allow(unused_imports)]
use super::*;

/// Strip ASCII control characters except newline/tab, trim, and cap length.
pub fn sanitize_string(input: &str) -> String {
    let mut out: String = input
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect();

    out.truncate(MAX_SANITIZED_LENGTH);
    out.trim().to_string()
}
