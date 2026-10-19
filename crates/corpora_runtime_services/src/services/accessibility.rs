//! App accessibility foundation on top of gpui's accesskit integration.
//! gpui owns the platform bridge: each window builds an accesskit tree every
//! frame and dispatches assistive-tech actions back to elements. Live state
//! is read via [`snapshot`], recomputed by [`refresh`].
//!
//! Element code should prefer gpui's own builders on
//! [`StatefulInteractiveElement`]: `role`, `aria_label`, `aria_description`,
//! `aria_keyshortcuts`, `aria_selected`, `aria_expanded`, `aria_toggled`,
//! `aria_value`, `aria_numeric_value` (+ min/max/step), `aria_placeholder`,
//! `aria_orientation`, `aria_level`, `aria_position_in_set`,
//! `aria_size_of_set`, `aria_row/column_index/count`, `aria_active_descendant`,
//! and `on_a11y_action`. A node is announced only when it has both an
//! `.id(...)` and a `.role(...)`; focused elements are announced when they
//! also `.track_focus(&handle)`. [`A11yExt`] adds the app-level helpers.

use gpui::{App, Global, Role, SharedString, StatefulInteractiveElement, accesskit::Live};

use crate::capabilities::CapabilityStatus;

#[cfg(test)]
#[path = "accessibility.test/mod.rs"]
mod accessibility_test;

mod a11y_ext;
mod accessibility_snapshot;
mod capability_status;
mod initialize;
mod observe;
mod refresh;
mod snapshot;
mod t;

pub use a11y_ext::A11yExt;
pub use accessibility_snapshot::AccessibilitySnapshot;
pub use capability_status::capability_status;
pub use initialize::initialize;
use observe::observe;
pub use refresh::refresh;
pub use snapshot::snapshot;
