#[allow(unused_imports)]
use super::*;

pub fn section(path: &str) -> &'static str {
    match path.split('/').nth(1).unwrap_or_default() {
        "explore" => "/explore",
        "projects" => "/projects",
        "ui" => "/ui",
        "settings" => "/settings",
        "notifications" => "/notifications",
        "profile" => "/profile/information",
        _ => "/",
    }
}
