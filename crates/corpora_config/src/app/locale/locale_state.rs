#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct LocaleState(pub SharedString);

impl Global for LocaleState {}
