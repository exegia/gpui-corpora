#[allow(unused_imports)]
use super::*;

pub(crate) const DEFAULT_MANIFEST_URL: &str = match option_env!("GPUI_UPDATE_MANIFEST_URL") {
    Some(url) => url,
    None => "https://releases.example.com/manifest.json",
};
