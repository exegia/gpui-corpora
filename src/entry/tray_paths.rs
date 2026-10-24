#[allow(unused_imports)]
use super::*;

pub(crate) const TRAY_PATHS: [Option<&str>; 4] = [
    None,
    Some(navigation::HOME),
    Some(navigation::EXPLORE),
    Some(navigation::PROJECTS),
];
