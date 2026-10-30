#[allow(unused_imports)]
use super::*;

pub(crate) const DEFAULT_MANIFEST_URL: Option<&str> = match option_env!("GPUI_UPDATE_MANIFEST_URL")
{
    Some(url) if !url.is_empty() => Some(url),
    _ => None,
};
