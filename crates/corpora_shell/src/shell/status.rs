use ely_gpui_component::{
    primitives::IconName,
    shell::{Connectivity, OfflineIndicator, StatusBar, StatusBarItem},
};
use gpui::{AnyElement, App, StyleRefinement, Window, prelude::*, px};

use super::chrome::style_chrome;

use crate::{
    connectivity::{self, ConnectivityState},
    services::updater,
    session::{self, SessionState},
};

mod bar;
mod frame_time_label;
mod update_label;

pub use bar::bar;
use frame_time_label::frame_time_label;
use update_label::update_label;
