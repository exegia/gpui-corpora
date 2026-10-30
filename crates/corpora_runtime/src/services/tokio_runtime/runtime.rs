#[allow(unused_imports)]
use super::*;

/// Dedicated tokio runtime for I/O-bound work: GPUI's executor is not a tokio
/// runtime, so tokio-dependent code must run via `runtime.spawn(...)` here.
pub struct TokioRuntime {
    pub runtime: Arc<tokio::runtime::Runtime>,
    pub http_client: reqwest::Client,
}

impl TokioRuntime {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("gpui-io")
            .build()
            .expect("failed to create tokio runtime");
        tracing::info!(target: "corpora::tokio_runtime", "tokio runtime created");

        // Build the reqwest client inside the runtime context so the connector
        // can resolve DNS via tokio.
        let _guard = runtime.enter();
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("failed to build reqwest client");

        Self {
            runtime: Arc::new(runtime),
            http_client,
        }
    }
}

impl Default for TokioRuntime {
    fn default() -> Self {
        Self::new()
    }
}
