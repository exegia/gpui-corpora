#[allow(unused_imports)]
use super::*;

#[cfg(feature = "otlp")]
#[test]
pub(crate) fn remote_sink_connects_with_tokio_runtime() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime for test");
    let sink = RemoteSink::new("http://localhost:9999", Some(rt.handle().clone()));
    assert!(sink.connected);
}
