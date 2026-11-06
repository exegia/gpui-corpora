#[allow(unused_imports)]
use super::*;

/// Validate that a deep-link URL uses the expected scheme (`corpora://`)
/// and reject unexpected hosts.
pub fn validate_deep_link_url(url: &str) -> Result<Url, AppError> {
    let parsed =
        Url::parse(url).map_err(|err| AppError::invalid_deep_link(url, err.to_string()))?;

    if parsed.scheme() != APP_URL_SCHEME {
        return Err(AppError::invalid_deep_link(
            url,
            format!("unsupported scheme `{}`", parsed.scheme()),
        ));
    }

    let host = parsed.host_str().unwrap_or_default();
    if !VALID_HOSTS.contains(&host) {
        return Err(AppError::invalid_deep_link(
            url,
            format!("unexpected host `{host}`"),
        ));
    }

    Ok(parsed)
}
