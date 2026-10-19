#[allow(unused_imports)]
use super::*;

/// One callback shared by every toast, mirroring user-notify's single
/// manager callback; the `tag` field tells responses apart.
pub type ResponseSink = Arc<OnceLock<Box<dyn Fn(ToastResponse) + Send + Sync + 'static>>>;
