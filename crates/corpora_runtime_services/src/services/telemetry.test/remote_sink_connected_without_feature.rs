#[allow(unused_imports)]
use super::*;

// The tracing-only install path (always connected) only exists without `otlp`;
// with the feature, installation needs a tokio runtime, covered by the test below.
#[cfg(not(feature = "otlp"))]
#[test]
pub(crate) fn remote_sink_connected_without_feature() {
    let sink = RemoteSink::new("http://localhost:9999", None);
    assert!(sink.connected);
}
