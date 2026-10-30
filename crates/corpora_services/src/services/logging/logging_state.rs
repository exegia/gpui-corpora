#[allow(unused_imports)]
use super::*;

pub struct LoggingState {
    pub runtime: LoggingRuntime,
    // Must stay held: dropping the WorkerGuard stops the non-blocking writer.
    #[allow(dead_code)]
    pub(crate) guard: Option<WorkerGuard>,
}

impl Global for LoggingState {}
