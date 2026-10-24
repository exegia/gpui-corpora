use super::*;
use std::fs;

// === split-modules: generated submodules ===
mod file_path_accepts_within_allowed_dir;
mod file_path_rejects_escape_from_allowed_dir;
mod file_path_rejects_nonexistent;
mod file_path_rejects_traversal;
mod invalid_notification_ids;
mod rejects_malformed_url;
mod rejects_unexpected_host;
mod rejects_wrong_scheme;
mod sanitize_keeps_newline_and_tab;
mod sanitize_strips_control_chars;
mod sanitize_trims_and_limits_length;
mod valid_deep_link_urls;
mod valid_notification_ids;
