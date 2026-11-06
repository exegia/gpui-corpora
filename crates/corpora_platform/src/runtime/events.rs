use gpui::{App, Global};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, ids::EventId, ipc::command::ForwardedCommand, time::AppTimestamp};

#[cfg(test)]
#[path = "events.test/mod.rs"]
mod events_test;

mod app_event;
mod app_event_kind;
mod app_event_queue;
mod drain;
mod emit;
mod emit_error;

pub use app_event::AppEvent;
pub use app_event_kind::AppEventKind;
pub use app_event_queue::AppEventQueue;
pub use drain::drain;
pub use emit::emit;
pub use emit_error::emit_error;
