use gpui::{App, BorrowAppContext as _, Global};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod default_directives;
mod env_filter;
mod initialize;
mod initialize_state;
mod logging_runtime;
mod logging_state;
mod shutdown;
mod snapshot;

use default_directives::DEFAULT_DIRECTIVES;
use env_filter::env_filter;
pub use initialize::initialize;
use initialize_state::initialize_state;
pub use logging_runtime::LoggingRuntime;
pub use logging_state::LoggingState;
pub use shutdown::shutdown;
pub use snapshot::snapshot;
