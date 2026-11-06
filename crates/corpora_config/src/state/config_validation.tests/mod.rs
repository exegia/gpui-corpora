//! Unit tests for [`super::validate_config`]: each test mutates one field of
//! an otherwise-default [`AppConfig`] and asserts the matching lint fires.
use super::*;
use crate::state::config_store::{AppConfig, PersistedWindowBounds};
#[allow(unused_imports)]
use std::collections::HashSet;

mod beta_update_channel_is_clean;
mod default_config_produces_no_lints;
mod empty_locale_fires_a_locale_lint;
mod empty_update_channel_fires_a_channel_lint;
mod has_lint_for;
mod huge_window_fires_a_lint;
mod known_granted_permissions_are_clean;
mod known_locale_en_is_clean;
mod known_locale_zh_cn_is_clean;
mod lint_severity_labels_are_stable;
mod multiple_bad_fields_stack_into_one_vec;
mod nan_dimension_fires_an_error_lint;
mod negative_height_fires_an_error_lint;
mod negative_origin_is_a_warning_not_error;
mod negative_width_fires_an_error_lint;
mod none_window_bounds_is_clean;
mod relative_active_path_fires_a_path_lint;
mod stable_update_channel_is_clean;
mod tiny_window_fires_a_width_and_height_lint;
mod unknown_granted_permission_fires_an_info_lint;
mod unknown_locale_fires_a_locale_lint;
mod unknown_update_channel_fires_a_channel_lint;

use has_lint_for::has_lint_for;
