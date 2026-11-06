use gpui::{App, BorrowAppContext, Global};
use serde::{Deserialize, Serialize};

use crate::{ids::EventId, time::AppTimestamp};

#[cfg(test)]
#[path = "error_surface.test/mod.rs"]
mod error_surface_test;

mod dismiss;
mod error_action;
mod error_category;
mod error_record;
mod error_surface_state;
mod initialize;
mod latest;
mod latest_message;
mod max_records;
mod persist_error;
mod record_count;
mod report;
mod scrub_home;
mod scrub_home_with;

pub use dismiss::dismiss;
pub use error_action::ErrorAction;
pub use error_category::ErrorCategory;
pub use error_record::ErrorRecord;
pub use error_surface_state::ErrorSurfaceState;
pub use initialize::initialize;
pub use latest::latest;
pub use latest_message::latest_message;
use max_records::MAX_RECORDS;
use persist_error::persist_error;
pub use record_count::record_count;
pub use report::report;
use scrub_home::scrub_home;
use scrub_home_with::scrub_home_with;
