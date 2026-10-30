#[allow(unused_imports)]
use super::*;

pub fn open_url(url: &str, cx: &mut App) -> Result<(), DesktopActionError> {
    // Only web URLs are ever expected here; rejecting other schemes keeps
    // `xdg-open` from being aimed at files, or custom/fictional handlers.
    let scheme_ok = {
        let (scheme, rest) = url.split_once(':').unwrap_or(("", ""));
        !rest.is_empty() && matches!(scheme, "http" | "https")
    };
    let result = if scheme_ok {
        open::that_detached(url).map_err(|source| DesktopActionError::OpenUrlFailed {
            url: url.to_string(),
            source,
        })
    } else {
        Err(DesktopActionError::OpenUrlFailed {
            url: url.to_string(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "only http/https URLs are supported",
            ),
        })
    };
    update_result("open_url", result.as_ref().err().map(|e| e.to_string()), cx);
    result
}
