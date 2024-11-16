use crate::version::RSVersion;

#[derive(Debug, PartialEq, Clone)]
pub struct RSLangConfig {
    pub version: RSVersion,
    pub derive: Vec<&'static str>,
}

impl RSLangConfig {
    pub fn new(version: RSVersion) -> Self {
        Self {
            version,
            ..Default::default()
        }
    }
}

impl Default for RSLangConfig {
    fn default() -> Self {
        Self {
            version: RSVersion::Latest,
            derive: vec![
                "Default",
                "Debug",
                "Clone",
                "PartialEq",
                "Eq",
                "Hash",
                "Serialize",
                "Deserialize",
            ],
        }
    }
}
