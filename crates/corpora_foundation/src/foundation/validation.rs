use std::path::{Path, PathBuf};

use thiserror::Error;
use url::Url;

use crate::errors::AppError;
use crate::navigation::deep_link::{APP_URL_SCHEME, VALID_HOSTS};

#[cfg(test)]
#[path = "validation.test/mod.rs"]
mod validation_test;

mod max_sanitized_length;
mod sanitize_string;
mod validate_deep_link_url;
mod validate_file_path;
mod validate_notification_id;
mod validation_error;

pub use max_sanitized_length::MAX_SANITIZED_LENGTH;
pub use sanitize_string::sanitize_string;
pub use validate_deep_link_url::validate_deep_link_url;
pub use validate_file_path::validate_file_path;
pub use validate_notification_id::validate_notification_id;
pub use validation_error::ValidationError;
