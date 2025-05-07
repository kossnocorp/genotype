use crate::version::PYVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfigLang {
    pub version: PYVersion,
}

impl PyConfigLang {
    pub fn new(version: PYVersion) -> Self {
        Self { version }
    }
}

impl Default for PyConfigLang {
    fn default() -> Self {
        Self {
            version: PYVersion::Latest,
        }
    }
}
