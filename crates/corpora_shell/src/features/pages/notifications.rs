use ely_gpui_component::{
    feedback::{EmptyState, Notification, NotificationCenter},
    primitives::IconName,
};
use gpui::{prelude::*, *};

use crate::notifications::inbox::{self, NotificationInboxItem};

mod notification;
mod notifications_page;

use notification::notification;
pub use notifications_page::NotificationsPage;
