#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct LifecycleState {
    pub stage: LifecycleStage,
    pub updated_at: AppTimestamp,
    pub startup_step: Option<String>,
    pub shutdown_step: Option<String>,
    pub last_startup_error: Option<String>,
    pub last_shutdown_error: Option<String>,
    pub last_error: Option<String>,
}

impl Global for LifecycleState {}

impl Default for LifecycleState {
    fn default() -> Self {
        Self::starting()
    }
}

impl LifecycleState {
    pub fn starting() -> Self {
        Self {
            stage: LifecycleStage::Starting,
            updated_at: AppTimestamp::now(),
            startup_step: None,
            shutdown_step: None,
            last_startup_error: None,
            last_shutdown_error: None,
            last_error: None,
        }
    }
}
