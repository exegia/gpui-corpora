//! Snapshot tests (insta, inline snapshots) for core serialisable structures.
//! `cargo insta review` rewrites the inline bodies after intentional changes.

use std::collections::HashSet;

use gpui_starter::app_state::{AppConfig, PersistedWindowBounds};
use gpui_starter::routes::AppRoute;
use gpui_starter::services::updater::{
    PlatformAsset, UpdateManifest, UpdateSnapshot, UpdateStatus,
};
use gpui_starter::sidebar::Page;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 1. Default AppConfig serialisation
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 2. AppConfig round-trip (serialize -> deserialize -> compare)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 3. Parse valid deep links
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 4. Parse invalid deep links
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 5. Theme file structure (keys / values)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 6. UpdateManifest serialization
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 7. UpdateStatus serialization
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// 8. UpdateSnapshot serialization
// ---------------------------------------------------------------------------

mod test_config_default_serialization;
mod test_config_roundtrip;
mod test_route_parse_invalid;
mod test_route_parse_valid;
mod test_theme_structure;
mod test_update_manifest_serialization;
mod test_update_snapshot_serialization;
mod test_update_status_serialization;
mod theme_dir;

use test_config_default_serialization::test_config_default_serialization;
use test_config_roundtrip::test_config_roundtrip;
use test_route_parse_invalid::test_route_parse_invalid;
use test_route_parse_valid::test_route_parse_valid;
use test_theme_structure::test_theme_structure;
use test_update_manifest_serialization::test_update_manifest_serialization;
use test_update_snapshot_serialization::test_update_snapshot_serialization;
use test_update_status_serialization::test_update_status_serialization;
use theme_dir::theme_dir;
