#[allow(unused_imports)]
use super::*;

/// Router path for a validated deep link. Query parameters are ignored, so
/// unsanitized query data never reaches navigation.
pub fn parse_deep_link(input: &str) -> Result<String, AppError> {
    let url =
        Url::parse(input).map_err(|err| AppError::invalid_deep_link(input, err.to_string()))?;

    if url.scheme() != APP_URL_SCHEME {
        return Err(AppError::invalid_deep_link(
            input,
            format!(
                "unsupported scheme `{}`, expected `{APP_URL_SCHEME}`",
                url.scheme()
            ),
        ));
    }

    let host = url.host_str().unwrap_or_default();
    if !VALID_HOSTS.contains(&host) {
        return Err(AppError::invalid_deep_link(
            input,
            format!("unexpected host `{host}`"),
        ));
    }

    // Validate decoded text: `url` percent-encodes `\0`, `/` and spaces, which
    // would otherwise reach the router path unchecked.
    let mut segments = Vec::new();
    for raw in url
        .path_segments()
        .into_iter()
        .flatten()
        .filter(|s| !s.is_empty())
    {
        let segment = percent_decode_str(raw)
            .decode_utf8()
            .map_err(|err| AppError::invalid_deep_link(input, err.to_string()))?;
        validate_path_segment(&segment)?;
        segments.push(segment.into_owned());
    }

    if host == "home" {
        return if segments.is_empty() {
            Ok(crate::navigation::HOME.to_string())
        } else {
            Err(AppError::invalid_deep_link(input, "home takes no path"))
        };
    }
    let mut path = format!("/{host}");
    for segment in segments {
        path.push('/');
        path.push_str(&segment);
    }
    Ok(path)
}
