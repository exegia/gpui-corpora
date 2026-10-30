#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct ConnectivitySnapshot {
    pub state: ConnectivityState,
    pub probe_url: String,
    pub interfaces: Vec<String>,
    pub last_error: Option<String>,
}

impl Default for ConnectivitySnapshot {
    fn default() -> Self {
        Self {
            state: ConnectivityState::Unknown,
            probe_url: "https://example.com".to_string(),
            interfaces: Vec::new(),
            last_error: None,
        }
    }
}

impl Global for ConnectivitySnapshot {}
