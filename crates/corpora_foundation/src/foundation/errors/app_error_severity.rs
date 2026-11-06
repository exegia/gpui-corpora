#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppErrorSeverity {
    Info,
    Warning,
    Error,
}
