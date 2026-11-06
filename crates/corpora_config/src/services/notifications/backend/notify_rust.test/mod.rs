use std::fs;
use std::path::{Path, PathBuf};

use super::*;

// === split-modules: generated submodules ===
mod candidates_probe_share_hicolor_then_local_hicolor_then_resources;
mod probe_falls_back_to_theme_name_when_nothing_is_bundled;
mod probe_finds_share_layout_beside_bin;
mod probe_prefers_install_layout_over_local_and_resources;
mod probe_prefers_smaller_size_within_a_layout;
mod stage_png;
mod temp_root;

use stage_png::stage_png;
use temp_root::temp_root;
