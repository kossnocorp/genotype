use crate::version::PyVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum PyPackageManager {
    #[default]
    #[serde(rename = "poetry")]
    Poetry,
    #[serde(rename = "uv")]
    Uv,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfigLang {
    pub version: PyVersion,
    #[serde(default)]
    pub manager: PyPackageManager,
}

impl PyConfigLang {
    pub fn new(version: PyVersion) -> Self {
        Self {
            version,
            manager: PyPackageManager::Poetry,
        }
    }

    pub fn new_with_manager(version: PyVersion, manager: PyPackageManager) -> Self {
        Self { version, manager }
    }
}

impl Default for PyConfigLang {
    fn default() -> Self {
        Self {
            version: PyVersion::Latest,
            manager: PyPackageManager::Poetry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_manager_is_poetry() {
        let config = PyConfigLang::default();
        assert_eq!(config.manager, PyPackageManager::Poetry);
    }

    #[test]
    fn test_new_with_manager() {
        let config = PyConfigLang::new_with_manager(PyVersion::Latest, PyPackageManager::Uv);
        assert_eq!(config.manager, PyPackageManager::Uv);
    }
}
