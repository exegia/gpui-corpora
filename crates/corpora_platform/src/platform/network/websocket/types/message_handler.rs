#[allow(unused_imports)]
use super::*;

pub type MessageHandler = Box<dyn Fn(&str) + Send + Sync>;
