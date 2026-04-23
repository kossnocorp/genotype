use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct TsPkgPath(GtpDistDirRelativePath);

impl GtlConfigPkgPathSetting for TsPkgPath {
    const DEFAULT: &'static str = "ts";

    fn path(&self) -> &GtpDistDirRelativePath {
        &self.0
    }
}

impl Default for TsPkgPath {
    fn default() -> Self {
        TsPkgPath(Self::default_relative_path())
    }
}
