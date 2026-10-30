#[allow(unused_imports)]
use super::*;

/// Set by the panic hook; the next render swaps in the error boundary view.
pub(crate) static RENDER_PANIC_OCCURRED: AtomicBool = AtomicBool::new(false);
