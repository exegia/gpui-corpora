use gpui::{App, Global};
use serde::{Deserialize, Serialize};

use crate::{
    app_state, ids::NotificationId, notifications::NotificationBackendKind, time::AppTimestamp,
};

mod clear_all;
mod initialize;
mod mark_all_read;
mod max_inbox_items;
mod notification_attempt_record;
mod notification_inbox_item;
mod notification_inbox_kind;
mod notification_inbox_state;
mod record;
mod record_attempt;
mod snapshot;
mod unread_count;

pub use clear_all::clear_all;
pub use initialize::initialize;
pub use mark_all_read::mark_all_read;
use max_inbox_items::MAX_INBOX_ITEMS;
pub use notification_attempt_record::NotificationAttemptRecord;
pub use notification_inbox_item::NotificationInboxItem;
pub use notification_inbox_kind::NotificationInboxKind;
pub use notification_inbox_state::NotificationInboxState;
pub use record::record;
pub use record_attempt::record_attempt;
pub use snapshot::snapshot;
pub use unread_count::unread_count;
