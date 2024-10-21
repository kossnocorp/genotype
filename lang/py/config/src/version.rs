use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PYVersion {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "latest")]
    Latest,
}

impl PYVersion {
    pub fn as_dependency_str(&self) -> &str {
        match self {
            Self::Legacy => r#"python = "^3.8""#,
            Self::Latest => r#"python = "^3.12""#,
        }
    }
}

impl Default for PYVersion {
    fn default() -> Self {
        Self::Latest
    }
}
