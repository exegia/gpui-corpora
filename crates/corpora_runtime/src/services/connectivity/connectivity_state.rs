#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectivityState {
    Unknown,
    Online,
    Offline,
    CaptiveOrFiltered,
}
