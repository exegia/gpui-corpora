#[allow(unused_imports)]
use super::*;

/// `true` when `id` is non-empty ASCII alphanumerics and hyphens.
pub fn validate_notification_id(id: &str) -> bool {
    !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}
