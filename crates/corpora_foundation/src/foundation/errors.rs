use std::{io, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod app_error;
mod app_error_severity;

pub use app_error::AppError;
pub use app_error_severity::AppErrorSeverity;
