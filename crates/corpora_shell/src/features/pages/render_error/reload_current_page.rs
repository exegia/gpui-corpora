#[allow(unused_imports)]
use super::*;

/// Clear the error boundary and retry rendering the active page.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = app, no_json)]
pub struct ReloadCurrentPage;
