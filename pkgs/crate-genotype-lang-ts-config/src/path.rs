use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct TsPkgPath(GtDistRelativePath);

impl GtlConfigPkgPathSetting for TsPkgPath {
    const DEFAULT: &'static str = "ts";

    fn path<'a>(&'a self) -> &'a GtDistRelativePath {
        &self.0
    }
}

impl Default for TsPkgPath {
    fn default() -> Self {
        TsPkgPath(Self::default_relative_path())
    }
}
