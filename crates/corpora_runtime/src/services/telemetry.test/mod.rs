use super::*;
use std::sync::{Arc, Mutex};

// === split-modules: generated submodules ===
mod fake_sink;
mod fake_sink_records_calls;
mod redacts_endpoint_host_only;
mod remote_sink_connected_without_feature;
mod remote_sink_connects_with_tokio_runtime;
mod remote_sink_without_runtime_stays_disconnected;
mod resolve_endpoint_falls_back_to_default;
mod resolve_endpoint_ignores_empty_explicit;
mod resolve_endpoint_prefers_explicit;

use fake_sink::FakeSink;
#[cfg(not(feature = "otlp"))]
use remote_sink_connected_without_feature::remote_sink_connected_without_feature;
