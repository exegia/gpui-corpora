//! Async-stream-to-GPUI plumbing: drive an async [`Stream`] of tokens into a
//! [`flume::Receiver`] on the shared tokio runtime.

use flume::Receiver;
use futures_util::Stream;
use gpui::{App, AsyncApp, Context, Task, WeakEntity};

mod channel_capacity;
mod producer_task;
mod spawn_token_poller;
mod spawn_token_stream;

use channel_capacity::CHANNEL_CAPACITY;
use producer_task::producer_task;
pub use spawn_token_poller::spawn_token_poller;
pub use spawn_token_stream::spawn_token_stream;
