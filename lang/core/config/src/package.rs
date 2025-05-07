use std::path::PathBuf;

/// Out target package setting trait. Its primary purpose is to provide a default path for config
/// during initialization and parse.
pub trait GtlConfigPackagePathSetting: Default {
    const DEFAULT: &'static str;

    fn pathbuf<'a>(&'a self) -> &'a PathBuf;

    fn to_path(&self) -> GtlConfigPackagePath {
        GtlConfigPackagePath(self.pathbuf().clone())
    }

    fn default_pathbuf() -> PathBuf {
        PathBuf::from(Self::DEFAULT)
    }
}

/// Target out path. It is relative to the dist path, e.g. "ts/src/index.ts".
pub struct GtlConfigPackagePath(PathBuf);

impl GtlConfigPackagePath {
    // pub fn new(path: PathBuf) -> Self {
    //     GtlConfigOutPath(path)
    // }

    pub fn join(&self, path: PathBuf) -> Self {
        GtlConfigPackagePath(self.0.join(path))
    }
}
