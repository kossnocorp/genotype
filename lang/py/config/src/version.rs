use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PYVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "latest")]
    Latest,
}

impl PYVersion {
    pub fn version_str(&self) -> &str {
        match self {
            Self::Legacy => "^3.8",
            Self::Latest => "^3.12",
        }
    }
}

impl Default for PYVersion {
    fn default() -> Self {
        Self::Latest
    }
}
