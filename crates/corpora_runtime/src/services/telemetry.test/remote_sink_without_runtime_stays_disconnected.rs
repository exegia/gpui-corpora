#[allow(unused_imports)]
use super::*;

#[cfg(feature = "otlp")]
#[test]
pub(crate) fn remote_sink_without_runtime_stays_disconnected() {
    // Installing without a runtime handle used to panic (batch exporter needs
    // a reactor); it must degrade to a disconnected sink instead.
    let sink = RemoteSink::new("http://localhost:9999", None);
    assert!(!sink.connected);
}
