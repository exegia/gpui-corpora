#[allow(unused_imports)]
use super::*;

pub(crate) fn read_persisted(state_file: &std::path::Path) -> AppConfig {
    let json = std::fs::read_to_string(state_file)
        .unwrap_or_else(|e| panic!("read {}: {e}", state_file.display()));
    serde_json::from_str(&json).expect("persisted state must parse back")
}
