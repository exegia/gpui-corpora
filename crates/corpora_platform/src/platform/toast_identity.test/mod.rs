use std::path::PathBuf;

use super::{ensure_shortcut_at, same_path};

// === split-modules: generated submodules ===
mod fake_exe;
mod read_back;
mod shortcut_is_refreshed_when_aumid_changes;
mod shortcut_is_refreshed_when_exe_moves;
mod shortcut_is_written_and_reused;
mod temp_programs;

use fake_exe::fake_exe;
use read_back::read_back;
use shortcut_is_refreshed_when_aumid_changes::shortcut_is_refreshed_when_aumid_changes;
use shortcut_is_refreshed_when_exe_moves::shortcut_is_refreshed_when_exe_moves;
use shortcut_is_written_and_reused::shortcut_is_written_and_reused;
use temp_programs::temp_programs;
