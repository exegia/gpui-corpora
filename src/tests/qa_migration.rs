//! Kit 0.7.3 migration invariants the compiler cannot check: manifest pins,
//! the single-kit lock graph (the round-1 links-conflict failure mode), the
//! gpui-form re-pin to the migrated freeoxide repo, and the kit-facade
//! import surface.

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 1. Manifest pins
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 2. Single-kit lock graph
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 3. gpui-form re-pin (freeoxide git rev, resolved unpatched)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 4. Kit facade import surface
// ---------------------------------------------------------------------------

mod app_sources;
mod assert_single_version;
mod block_name;
mod committed_file;
mod committed_lock_carries_no_removed_dependency;
mod gpui_form_pins_migrated_freeoxide_repo;
mod gpui_form_resolves_from_freeoxide_git_unpatched;
mod gpui_pre_snapshot_family_is_unified;
mod lock_blocks;
mod lock_graph_holds_one_kit_family;
mod manifest_declares_only_justified_kit_deps;
mod repo_file;
mod satellites_stay_on_form_repo_registry_lines;
mod sources_reach_kit_layers_only_through_the_facade;

use app_sources::app_sources;
use assert_single_version::assert_single_version;
use block_name::block_name;
use committed_file::committed_file;
use committed_lock_carries_no_removed_dependency::committed_lock_carries_no_removed_dependency;
use gpui_form_pins_migrated_freeoxide_repo::gpui_form_pins_migrated_freeoxide_repo;
use gpui_form_resolves_from_freeoxide_git_unpatched::gpui_form_resolves_from_freeoxide_git_unpatched;
use gpui_pre_snapshot_family_is_unified::gpui_pre_snapshot_family_is_unified;
use lock_blocks::lock_blocks;
use lock_graph_holds_one_kit_family::lock_graph_holds_one_kit_family;
use manifest_declares_only_justified_kit_deps::manifest_declares_only_justified_kit_deps;
use repo_file::repo_file;
use satellites_stay_on_form_repo_registry_lines::satellites_stay_on_form_repo_registry_lines;
use sources_reach_kit_layers_only_through_the_facade::sources_reach_kit_layers_only_through_the_facade;
