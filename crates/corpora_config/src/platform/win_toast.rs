//! Interactive Windows toasts. user-notify 0.4's Windows backend emits
//! action-less toast XML, so this module hand-rolls the send with the
//! windows crate: action buttons, a quick-reply input, and the
//! Activated/Dismissed plumbing that reports through a shared sink.

use std::sync::{Arc, OnceLock};

use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::{IReference, TypedEventHandler};
use windows::UI::Notifications::{
    ToastActivatedEventArgs, ToastDismissalReason, ToastDismissedEventArgs, ToastNotification,
    ToastNotificationManager,
};
use windows::core::{HSTRING, IInspectable, Interface, Ref};

use crate::notifications::{
    ACTION_OPEN, ACTION_REPLY, ACTION_SNOOZE, CATEGORY_ACTIONS, CATEGORY_REPLY, DESKTOP_ENTRY_ID,
    NotificationRequest,
};

#[cfg(test)]
#[path = "win_toast.test/mod.rs"]
mod win_toast_test;

mod actions_xml;
mod activated_action;
mod escaped;
mod log;
mod message_group;
mod reply_input_id;
mod reply_text;
mod response_action;
mod response_sink;
mod send;
mod toast_response;
mod toast_tag;
mod toast_xml;

use actions_xml::actions_xml;
use activated_action::activated_action;
use escaped::escaped;
use log::LOG;
use message_group::MESSAGE_GROUP;
use reply_input_id::REPLY_INPUT_ID;
use reply_text::reply_text;
pub use response_action::ResponseAction;
pub use response_sink::ResponseSink;
pub use send::send;
pub use toast_response::ToastResponse;
use toast_tag::toast_tag;
use toast_xml::toast_xml;
