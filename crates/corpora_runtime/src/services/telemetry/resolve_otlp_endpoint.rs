#[allow(unused_imports)]
use super::*;

/// Explicit endpoint wins, then `OTEL_EXPORTER_OTLP_ENDPOINT`, then the default.
pub(crate) fn resolve_otlp_endpoint(explicit: Option<&str>) -> String {
    if let Some(ep) = explicit
        && !ep.trim().is_empty()
    {
        return ep.trim().to_owned();
    }
    match std::env::var(ENV_OTLP_ENDPOINT) {
        Ok(v) if !v.trim().is_empty() => v.trim().to_owned(),
        _ => DEFAULT_OTLP_ENDPOINT.to_owned(),
    }
}
