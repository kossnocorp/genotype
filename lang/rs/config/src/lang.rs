use crate::version::RSVersion;

#[derive(Debug, PartialEq, Clone)]
pub struct RSLangConfig {
    pub version: RSVersion,
}

impl RSLangConfig {
    pub fn new(version: RSVersion) -> Self {
        Self { version }
    }
}

impl Default for RSLangConfig {
    fn default() -> Self {
        Self {
            version: RSVersion::Latest,
        }
    }
}
