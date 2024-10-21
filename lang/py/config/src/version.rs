use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PYVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "latest")]
    Latest,
}

impl Default for PYVersion {
    fn default() -> Self {
        Self::Latest
    }
}
