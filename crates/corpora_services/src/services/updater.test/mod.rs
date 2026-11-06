use super::types::{DEFAULT_MANIFEST_URL, current_app_version, pending_swap_path, platform_key};
use super::*;

// === split-modules: generated submodules ===
mod current_app_version_matches_cargo_pkg;
mod default_manifest_url_not_empty;
mod manifest_parse_invalid_json;
mod manifest_parse_minimal_version_only;
mod manifest_parse_valid_full;
mod pending_swap_path_is_consistent;
mod platform_key_format;
mod semver_equal;
mod semver_invalid_version;
mod semver_newer_available;
mod semver_older_manifest;
mod unconfigured_manifest_fails_without_network;
mod update_status_default_is_idle;
