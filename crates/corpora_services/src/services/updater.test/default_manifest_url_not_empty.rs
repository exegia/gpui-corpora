#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn default_manifest_url_not_empty() {
    if let Some(url) = DEFAULT_MANIFEST_URL {
        assert!(!url.is_empty());
    } else {
        assert!(option_env!("GPUI_UPDATE_MANIFEST_URL").is_none_or(str::is_empty));
    }
}
