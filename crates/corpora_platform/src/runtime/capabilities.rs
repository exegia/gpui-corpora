use std::collections::BTreeMap;

use gpui::{App, Global, SharedString};
use serde::{Deserialize, Serialize};

mod capability_registry;
mod capability_status;
mod initialize;
mod set;
mod snapshot;

pub use capability_registry::CapabilityRegistry;
pub use capability_status::CapabilityStatus;
pub use initialize::initialize;
pub use set::set;
pub use snapshot::snapshot;
