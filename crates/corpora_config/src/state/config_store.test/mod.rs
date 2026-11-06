#[allow(unused_imports)]
use std::sync::Mutex;

use gpui::TestAppContext;
use tempfile::tempdir;

use super::*;

// ---------------------------------------------------------------------------
// Write-path decision functions (prepare_flush / commit_flush)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Write-path orchestration (update_config / force_save / arm_debounce)
// ---------------------------------------------------------------------------

mod a_burst_of_updates_lands_as_the_final_value;
mod a_failed_write_is_not_retried_until_the_next_mutation;
mod an_unchanged_update_skips_the_write_entirely;
mod clean_state;
mod clicked_notification_read_state_persists;
mod commit_flush_failure_keeps_the_state_dirty_for_retry;
mod commit_flush_remarks_dirty_when_config_changed_midflight;
mod commit_flush_success_commits_the_written_bytes;
mod corrupt_config_is_quarantined;
mod debounce_test_lock;
mod debounce_tests;
mod dirty_state;
mod force_save_flushes_immediately_and_cancels_the_debounce;
mod normalized_drops_implausible_window_bounds;
mod normalized_keeps_plausible_and_multimonitor_bounds;
mod oversized_config_is_quarantined_without_parsing;
mod prepare_flush_serializes_a_dirty_state_to_its_file;
mod prepare_flush_skips_unchanged_bytes_and_clears_dirty;
mod read_persisted;
mod save_and_load_config_uses_json_state_file;
mod update_config_before_initialize_warns_and_does_nothing;
mod update_config_persists_after_the_debounce_window;

use clean_state::clean_state;
use debounce_test_lock::debounce_test_lock;
use debounce_tests::DEBOUNCE_TESTS;
use dirty_state::dirty_state;
use read_persisted::read_persisted;
