//! Wire types for the typed IPC command/response protocol. Transport is one
//! compact JSON object per line over `crate::single_instance`'s local socket.

use serde::{Deserialize, Serialize};

mod forwarded_command;
mod forwarded_request;
mod forwarded_response;
mod log;

pub use forwarded_command::ForwardedCommand;
pub use forwarded_request::ForwardedRequest;
pub use forwarded_response::ForwardedResponse;
use log::LOG;
