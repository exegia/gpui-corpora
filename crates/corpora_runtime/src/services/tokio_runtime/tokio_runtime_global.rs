#[allow(unused_imports)]
use super::*;

pub struct TokioRuntimeGlobal(pub TokioRuntime);

impl Global for TokioRuntimeGlobal {}
