use crate::version::PYVersion;

#[derive(Debug, PartialEq, Clone)]
pub struct PYLangConfig {
    pub version: PYVersion,
}

impl PYLangConfig {
    pub fn new(version: PYVersion) -> Self {
        Self { version }
    }
}

impl Default for PYLangConfig {
    fn default() -> Self {
        Self {
            version: PYVersion::Latest,
        }
    }
}
