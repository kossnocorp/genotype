use crate::version::PyVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfigLang {
    pub version: PyVersion,
}

impl PyConfigLang {
    pub fn new(version: PyVersion) -> Self {
        Self { version }
    }
}

impl Default for PyConfigLang {
    fn default() -> Self {
        Self {
            version: PyVersion::Latest,
        }
    }
}
