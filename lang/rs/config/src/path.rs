use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct RsPkgPath(GtDistRelativePath);

impl GtlConfigPkgPathSetting for RsPkgPath {
    const DEFAULT: &'static str = "rs";

    fn path<'a>(&'a self) -> &'a GtDistRelativePath {
        &self.0
    }
}

impl Default for RsPkgPath {
    fn default() -> Self {
        RsPkgPath(Self::default_relative_path())
    }
}
