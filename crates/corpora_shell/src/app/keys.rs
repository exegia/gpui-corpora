use corpora_ui::ely::{
    theme::Platform,
    typography::keys::{keystroke, keystroke_labels},
};

#[cfg(test)]
#[path = "keys.test/mod.rs"]
mod keys_test;

mod label;
mod page;
mod toggle_search;

pub use label::label;
pub use page::page;
pub use toggle_search::TOGGLE_SEARCH;
