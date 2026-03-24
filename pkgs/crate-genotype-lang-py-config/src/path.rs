use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct PyPkgPath(GtDistRelativePath);

impl GtlConfigPkgPathSetting for PyPkgPath {
    const DEFAULT: &'static str = "py";

    fn path<'a>(&'a self) -> &'a GtDistRelativePath {
        &self.0
    }
}

impl Default for PyPkgPath {
    fn default() -> Self {
        PyPkgPath(Self::default_relative_path())
    }
}
