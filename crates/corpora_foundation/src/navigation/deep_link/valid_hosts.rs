#[allow(unused_imports)]
use super::*;

/// Top-level sections a deep link may open. `home` maps to `/`.
pub(crate) const VALID_HOSTS: &[&str] = &[
    "home",
    "explore",
    "projects",
    "settings",
    "profile",
    "notifications",
    "diagnostics",
    "about",
];
