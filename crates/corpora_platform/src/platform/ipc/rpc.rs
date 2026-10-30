//! Line adapters between serde types and the newline-JSON IPC framing used by
//! `crate::single_instance`.

use serde::de::DeserializeOwned;

use super::command::ForwardedRequest;

mod decode_line;
mod decode_request;
mod encode_line;
mod log;

pub use decode_line::decode_line;
pub use decode_request::decode_request;
pub use encode_line::encode_line;
use log::LOG;
