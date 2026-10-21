use corpora_ui::ely::{
    buttons::{Button, ButtonVariant},
    data_display::{Badge, Tone},
    feedback::EmptyState,
    layout::ScrollArea,
    primitives::{Icon, IconName},
    theme::{ActiveTheme, ControlSize, IconSize, TextSize},
    typography::RelativeTime,
};
use gpui::{prelude::*, *};

use crate::notifications::inbox::{self, NotificationInboxItem};

mod notification;
mod notifications_page;

use notification::notification;
pub use notifications_page::NotificationsPage;
