#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlatformAsset {
    pub url: String,
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub size: u64,
}
