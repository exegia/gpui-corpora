#[allow(unused_imports)]
use super::*;

/// Decode a line as a [`ForwardedRequest`]; `None` on blank or malformed
/// input (the latter is logged).
pub fn decode_request(line: &str) -> Option<ForwardedRequest> {
    match decode_line::<ForwardedRequest>(line) {
        Ok(Some(req)) => Some(req),
        Ok(None) => None,
        Err(err) => {
            tracing::warn!(
                target: LOG,
                error = %err,
                line = line.trim(),
                "failed to decode ipc request line"
            );
            None
        }
    }
}
