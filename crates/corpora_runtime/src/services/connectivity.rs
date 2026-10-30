use gpui::{App, BorrowAppContext as _, Global};
use network_interface::NetworkInterfaceConfig;
use serde::{Deserialize, Serialize};

mod check_now;
mod connectivity_snapshot;
mod connectivity_state;
mod initialize;
mod read_interfaces;
mod snapshot;

pub use check_now::check_now;
pub use connectivity_snapshot::ConnectivitySnapshot;
pub use connectivity_state::ConnectivityState;
pub use initialize::initialize;
use read_interfaces::read_interfaces;
pub use snapshot::snapshot;
