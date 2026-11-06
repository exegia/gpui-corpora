use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::errors::AppError;

mod app_paths;
mod ensure_parent_dir;
mod project_dirs;

pub use app_paths::AppPaths;
pub use ensure_parent_dir::ensure_parent_dir;
pub use project_dirs::project_dirs;
