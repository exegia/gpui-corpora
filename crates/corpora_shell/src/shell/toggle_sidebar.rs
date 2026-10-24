#[allow(unused_imports)]
use super::*;

#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct ToggleSidebar;
