use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpPaths {
    /// Absolute canonical cwd path.
    pub cwd: GtpCwdPath,
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

impl GtpPaths {
    pub fn try_new(config_file_path: GtpConfigFilePath, config: &GtpConfig) -> Result<Self> {
        let cwd = GtpCwdPath::try_new()?;
        let config_dir = config_file_path.to_config_dir_path();
        let root = config.root.to_cwd_relative_path(&config_dir).into();
        let dist = config.dist.to_cwd_relative_path(&root).into();
        let src = config.src.to_cwd_relative_path(&root).into();
        let entry = config.entry.to_cwd_relative_path(&src).into();

        Ok(Self {
            cwd,
            config_file: config_file_path,
            root,
            dist,
            src,
            entry,
        })
    }
}
