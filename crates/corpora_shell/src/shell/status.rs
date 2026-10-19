use ely_gpui_component::{
    primitives::IconName,
    shell::{Connectivity, OfflineIndicator, StatusBar, StatusBarItem},
};
use gpui::App;

use crate::{
    connectivity::{self, ConnectivityState},
    navigation,
    services::updater,
    session::{self, SessionState},
};

mod bar;
mod frame_time_label;
mod navigate_on_click;
mod update_label;

pub use bar::bar;
use frame_time_label::frame_time_label;
use navigate_on_click::navigate_on_click;
use update_label::update_label;
