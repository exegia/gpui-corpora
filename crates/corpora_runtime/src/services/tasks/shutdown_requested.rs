#[allow(unused_imports)]
use super::*;

pub(crate) static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
