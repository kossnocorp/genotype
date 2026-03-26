use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PyVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "latest")]
    Latest,
}

impl PyVersion {
    pub fn version_str(&self) -> &str {
        match self {
            Self::Legacy => "^3.8",
            Self::Latest => "^3.13",
        }
    }
}

impl Default for PyVersion {
    fn default() -> Self {
        Self::Latest
    }
}
