//! Telemetry service: opt-in telemetry with local-only and remote (OTLP) modes.

mod sink;

use std::sync::Arc;

use gpui::{App, BorrowAppContext as _, Global};
use opentelemetry::global;

use sink::{DisabledSink, LocalSink, RemoteSink};

#[cfg(test)]
#[path = "../telemetry.test/mod.rs"]
mod telemetry_test;

mod default_otlp_endpoint;
mod env_otlp_endpoint;
mod flush;
mod handle_record_result;
mod initialize;
mod record_error;
mod record_event;
mod redact_endpoint;
mod resolve_otlp_endpoint;
mod service_name;
mod set_capability;
mod set_mode;
mod set_user_property;
mod shutdown;
mod snapshot;
mod telemetry_error;
mod telemetry_mode;
mod telemetry_runtime;
mod telemetry_sink;
mod telemetry_snapshot;
mod with_runtime;

use default_otlp_endpoint::DEFAULT_OTLP_ENDPOINT;
use env_otlp_endpoint::ENV_OTLP_ENDPOINT;
pub use flush::flush;
use handle_record_result::handle_record_result;
pub use initialize::initialize;
pub use record_error::record_error;
pub use record_event::record_event;
use redact_endpoint::redact_endpoint;
use resolve_otlp_endpoint::resolve_otlp_endpoint;
#[cfg(feature = "otlp")]
use service_name::SERVICE_NAME;
use set_capability::set_capability;
pub use set_mode::set_mode;
pub use set_user_property::set_user_property;
pub use shutdown::shutdown;
pub use snapshot::snapshot;
pub use telemetry_error::TelemetryError;
pub use telemetry_mode::TelemetryMode;
use telemetry_runtime::TelemetryRuntime;
pub use telemetry_sink::TelemetrySink;
pub use telemetry_snapshot::TelemetrySnapshot;
use with_runtime::with_runtime;
