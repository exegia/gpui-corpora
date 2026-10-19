use ely_gpui_component::{
    theme::Platform,
    typography::keys::{keystroke, keystroke_labels},
};

#[cfg(test)]
#[path = "keys.test/mod.rs"]
mod keys_test;

mod label;
mod page;
mod toggle_search;
mod toggle_sidebar;

pub use label::label;
pub use page::page;
pub use toggle_search::TOGGLE_SEARCH;
pub use toggle_sidebar::TOGGLE_SIDEBAR;
