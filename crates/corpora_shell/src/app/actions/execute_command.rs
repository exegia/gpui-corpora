#[allow(unused_imports)]
use super::*;

#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = app, no_json)]
pub struct ExecuteCommand(pub crate::commands::CommandId);
