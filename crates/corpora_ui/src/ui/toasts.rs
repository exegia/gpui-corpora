//! The workspace's single Ely [`Toaster`], reachable from services that hold
//! no window handle.

use std::collections::HashMap;

use ely_gpui_component::feedback::{Toast, Toaster};
use gpui::{App, Entity, Global};

mod app_toaster;
mod install;
mod push;
mod replace;

use app_toaster::AppToaster;
pub use install::install;
pub use push::push;
pub use replace::replace;
