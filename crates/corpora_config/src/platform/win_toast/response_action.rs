#[allow(unused_imports)]
use super::*;

#[derive(Debug)]
pub enum ResponseAction {
    Default,
    Dismiss,
    Other(String),
}
