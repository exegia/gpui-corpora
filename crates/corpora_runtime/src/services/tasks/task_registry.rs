#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct TaskRegistry {
    pub tasks: Vec<BackgroundTask>,
}

impl Global for TaskRegistry {}
