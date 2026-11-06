//! Non-fatal config validation: lints an [`AppConfig`] into [`ConfigLint`]
//! warnings without ever rejecting the load.

use std::collections::HashSet;

use crate::app::{LOCALE_EN, LOCALE_ZH_CN};
use crate::state::config_store::{AppConfig, PersistedWindowBounds};

#[cfg(test)]
#[path = "config_validation.tests/mod.rs"]
mod config_validation_tests;

mod config_lint;
mod known_locales;
mod known_permissions;
mod known_update_channels;
mod lint_severity;
mod log_target;
mod max_window_dim;
mod min_window_dim;
mod validate_active_path;
mod validate_config;
mod validate_granted_permissions;
mod validate_locale;
mod validate_update_channel;
mod validate_window_bounds;

pub use config_lint::ConfigLint;
use known_locales::KNOWN_LOCALES;
use known_permissions::KNOWN_PERMISSIONS;
use known_update_channels::KNOWN_UPDATE_CHANNELS;
pub use lint_severity::LintSeverity;
use log_target::LOG_TARGET;
use max_window_dim::MAX_WINDOW_DIM;
use min_window_dim::MIN_WINDOW_DIM;
use validate_active_path::validate_active_path;
pub use validate_config::validate_config;
use validate_granted_permissions::validate_granted_permissions;
use validate_locale::validate_locale;
use validate_update_channel::validate_update_channel;
use validate_window_bounds::validate_window_bounds;
