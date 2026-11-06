#[allow(unused_imports)]
use super::*;

/// `corpora://` URL for a router path; the inverse of [`parse_deep_link`].
pub fn to_url(path: &str) -> String {
    match path.trim_start_matches('/') {
        "" => format!("{APP_URL_SCHEME}://home"),
        rest => format!("{APP_URL_SCHEME}://{rest}"),
    }
}
