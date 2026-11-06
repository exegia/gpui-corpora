#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    Network,
    Storage,
    Rendering,
    Config,
    System,
}

impl ErrorCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Storage => "storage",
            Self::Rendering => "rendering",
            Self::Config => "config",
            Self::System => "system",
        }
    }
}
