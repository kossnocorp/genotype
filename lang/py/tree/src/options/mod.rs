#[derive(Debug, PartialEq, Clone)]
pub enum PYVersion {
    Legacy,
    V3_12,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PYOptions {
    pub version: PYVersion,
}

impl PYOptions {
    pub fn new(version: PYVersion) -> Self {
        Self { version }
    }
}

impl Default for PYOptions {
    fn default() -> Self {
        Self {
            version: PYVersion::V3_12,
        }
    }
}
