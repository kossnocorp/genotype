use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpPaths {
    /// Config file path.
    pub config_file: GtpConfigFilePath,
    /// Root dir path.
    pub root: GtpRootDirPath,
    /// Dist dir path.
    pub dist: GtpDistDirPath,
    /// Src dir path.
    pub src: GtpSrcDirPath,
    /// Entry pattern path.
    pub entry: GtpEntryPattern,
}
