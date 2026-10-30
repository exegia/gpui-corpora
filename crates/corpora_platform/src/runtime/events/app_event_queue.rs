#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct AppEventQueue(pub Vec<AppEvent>);

impl Global for AppEventQueue {}
