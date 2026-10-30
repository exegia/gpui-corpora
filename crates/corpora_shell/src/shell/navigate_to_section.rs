#[allow(unused_imports)]
use super::*;

/// Open the route section at index `i` of [`navigation::SECTIONS`].
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = workspace, no_json)]
pub struct NavigateToSection(pub usize);
