#[allow(unused_imports)]
use super::*;

/// RUST_LOG wins where set; these directives are the floor.
pub(crate) fn env_filter() -> tracing_subscriber::EnvFilter {
    let mut filter = tracing_subscriber::EnvFilter::from_default_env();
    for directive in DEFAULT_DIRECTIVES {
        filter = filter.add_directive(directive.parse().expect("hardcoded directive is valid"));
    }
    filter
}
