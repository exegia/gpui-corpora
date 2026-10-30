use gpui::{App, BorrowAppContext as _, Global, SharedString};

mod delete_secret;
mod entry;
mod get_secret;
mod initialize;
mod secure_storage_error;
mod secure_storage_snapshot;
mod set_secret;
mod snapshot;
mod update_last_error;

pub use delete_secret::delete_secret;
use entry::entry;
pub use get_secret::get_secret;
pub use initialize::initialize;
pub use secure_storage_error::SecureStorageError;
pub use secure_storage_snapshot::SecureStorageSnapshot;
pub use set_secret::set_secret;
pub use snapshot::snapshot;
use update_last_error::update_last_error;
