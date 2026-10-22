use gpui::{AnyElement, App, IntoElement, WeakEntity};
use gpui_router::{Outlet, Route, Routes};

use crate::{
    pages,
    shell::{self, Workspace},
};

mod panes;
mod section;
pub mod ui;
mod view;

pub use panes::{Panes, panes};
pub use section::section;
pub use view::view;
