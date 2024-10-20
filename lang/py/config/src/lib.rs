use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PYVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "3.12")]
    V3_12,
}

impl Default for PYVersion {
    fn default() -> Self {
        Self::V3_12
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PYConfig {
    pub enabled: Option<bool>,
    pub version: Option<PYVersion>,
}

impl PYConfig {
    pub fn new(version: PYVersion) -> Self {
        Self {
            enabled: Some(true),
            version: Some(version),
        }
    }
}

impl Default for PYConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            version: None,
        }
    }
}
