#[cfg(feature = "otlp")]
use std::sync::OnceLock;

use super::TelemetryError;

mod disabled_sink;
mod install_otlp_tracer;
mod installed_pipeline;
mod local_sink;
mod remote_sink;

pub(super) use disabled_sink::DisabledSink;
#[cfg(feature = "otlp")]
pub(super) use install_otlp_tracer::install_otlp_tracer;
#[cfg(not(feature = "otlp"))]
pub(super) use install_otlp_tracer::install_otlp_tracer;
#[cfg(feature = "otlp")]
use installed_pipeline::INSTALLED_PIPELINE;
pub(super) use local_sink::LocalSink;
pub(super) use remote_sink::RemoteSink;
