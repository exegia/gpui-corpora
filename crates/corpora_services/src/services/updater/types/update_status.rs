#[allow(unused_imports)]
use super::*;

// Serialize is load-bearing: tests/snapshot_tests.rs pins these as yaml.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    Available {
        version: String,
        notes: String,
    },
    Downloading {
        progress: u32, // 0–100
    },
    Downloaded {
        version: String,
        path: String,
    },
    ReadyToInstall,
    Error(String),
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self::Idle
    }
}
