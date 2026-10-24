#[allow(unused_imports)]
use super::*;

/// Decode one JSON value from a line; blank lines yield `Ok(None)` so
/// keepalives can be skipped without a protocol error.
pub fn decode_line<T: DeserializeOwned>(line: &str) -> Result<Option<T>, serde_json::Error> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    serde_json::from_str(trimmed).map(Some)
}
