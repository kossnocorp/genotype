use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyModuleName(#[serde(default = "PyModuleName::default_module")] String);

impl PyModuleName {
    fn default_module() -> String {
        "module".to_string()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PyModuleName {
    fn default() -> Self {
        Self(Self::default_module())
    }
}
