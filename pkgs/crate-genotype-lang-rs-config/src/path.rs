use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct RsPkgPath(GtpDistDirRelativePath);

impl GtlConfigPkgPathSetting for RsPkgPath {
    const DEFAULT: &'static str = "rs";

    fn path(&self) -> &GtpDistDirRelativePath {
        &self.0
    }
}

impl Default for RsPkgPath {
    fn default() -> Self {
        RsPkgPath(Self::default_relative_path())
    }
}
