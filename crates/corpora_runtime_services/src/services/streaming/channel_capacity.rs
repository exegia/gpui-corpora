#[allow(unused_imports)]
use super::*;

/// Upper bound on buffered tokens before the producer waits on the consumer.
pub(crate) const CHANNEL_CAPACITY: usize = 1024;
