//! Accessibility contracts that hold without a live window: capability
//! registration truthfulness, snapshot semantics, the public element-helper
//! surface, checklist claims, and runtime-proven rules, all anchored in code.

use gpui_kit::{Role, accesskit::Live, div, prelude::*};

use gpui_starter::accessibility::{A11yExt, AccessibilitySnapshot, capability_status};
use gpui_starter::ui::widgets::virtual_list_item;

// ---------------------------------------------------------------------------
// Capability registration truthfulness
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Snapshot semantics
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Element helper surface
// ---------------------------------------------------------------------------

// A11y nodes are only observable through a live window with assistive tech
// attached, so these pin the public surface and its gpui interop.

// ---------------------------------------------------------------------------
// Checklist claims must match code
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Runtime-verified rules must match code
// ---------------------------------------------------------------------------

// Keyed ids prevent the duplicate-a11y-node-id panic; virtual_list's anchors
// pin the 0-based position contract. Both need a live window (see header).

mod a11y_helpers_are_public_and_chainable;
mod capability_registration_is_supported_and_enabled;
mod checklist_bridge_claims_anchor_in_code;
mod checklist_documents_the_real_bridge;
mod default_snapshot_without_windows;
mod playground_helpers_derive_ids_from_call_site_keys;
mod virtual_list_item_chains_with_selection_state;
mod virtual_list_pins_zero_based_positions_and_container_setsize;

use a11y_helpers_are_public_and_chainable::a11y_helpers_are_public_and_chainable;
use capability_registration_is_supported_and_enabled::capability_registration_is_supported_and_enabled;
use checklist_bridge_claims_anchor_in_code::checklist_bridge_claims_anchor_in_code;
use checklist_documents_the_real_bridge::checklist_documents_the_real_bridge;
use default_snapshot_without_windows::default_snapshot_without_windows;
use playground_helpers_derive_ids_from_call_site_keys::playground_helpers_derive_ids_from_call_site_keys;
use virtual_list_item_chains_with_selection_state::virtual_list_item_chains_with_selection_state;
use virtual_list_pins_zero_based_positions_and_container_setsize::virtual_list_pins_zero_based_positions_and_container_setsize;
