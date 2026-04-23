use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpPaths {
    /// Absolute canonical cwd path.
    pub cwd: GtpCwdPath,
    /// Root dir path relative to [GtpPaths::cwd].
    pub root: GtpRootDirPath,
    /// Dist dir path relative to [GtpPaths::root].
    pub dist: GtpDistDirPath,
    /// Src dir path relative to [GtpPaths::root].
    pub src: GtpSrcDirPath,
}

impl GtpPaths {
    pub fn try_new(config: &GtpConfig) -> Result<Self> {
        let cwd = GtpCwdPath::try_new()?;
        let root = GtpRootDirPath::new(config.root.clone());
        let dist = GtpDistDirPath::new(config.dist.clone());
        let src = GtpSrcDirPath::new(config.src.clone());
        Ok(Self {
            cwd,
            root,
            dist,
            src,
        })
    }
}
