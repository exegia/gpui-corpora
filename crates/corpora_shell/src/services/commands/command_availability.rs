#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct CommandAvailability {
    pub enabled: bool,
    pub disabled_reason: Option<SharedString>,
}
