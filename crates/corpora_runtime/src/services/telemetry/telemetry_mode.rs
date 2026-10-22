#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TelemetryMode {
    Disabled,
    LocalOnly,
    Remote,
}
