use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct PyPkgPath(GtpDistDirRelativePath);

impl GtlConfigPkgPathSetting for PyPkgPath {
    const DEFAULT: &'static str = "py";

    fn path(&self) -> &GtpDistDirRelativePath {
        &self.0
    }
}

impl Default for PyPkgPath {
    fn default() -> Self {
        PyPkgPath(Self::default_relative_path())
    }
}
