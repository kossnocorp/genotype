use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum RSVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "latest")]
    Latest,
}

impl RSVersion {
    pub fn as_dependency_str(&self) -> &str {
        match self {
            Self::Legacy => r#"rsthon = "^3.8""#,
            Self::Latest => r#"rsthon = "^3.12""#,
        }
    }
}

impl Default for RSVersion {
    fn default() -> Self {
        Self::Latest
    }
}
