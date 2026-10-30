#[allow(unused_imports)]
use super::*;

/// Encode a `Serialize`-able value as one compact JSON line with trailing
/// `\n`, the wire shape the single-instance forwarder expects.
pub fn encode_line<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut line = serde_json::to_string(value)?;
    line.push('\n');
    Ok(line)
}
