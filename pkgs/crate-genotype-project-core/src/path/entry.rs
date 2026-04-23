//! Entry paths. It is the forth level of paths, relative to [GtpSrcDirPath].
//!
//! Entry is the path pattern that allows to match modules. It is is defined in the config and
//! by default equals to `"**/*.type"`.
//!
//! Types:
//!
//! - [GtpEntryPatternPath]: Path pattern relative to [GtpSrcDirPath]. It encloses [GtpSrcDirRelativePath].

use crate::prelude::internal::*;

// region: Types

// region: Entry pattern

/// Entry pattern path relative to the source directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpEntryPatternPath(GtpSrcDirRelativePath);

impl GtpEntryPatternPath {
    pub fn with_parent(&self, path: &RelativePathBuf) -> GtpEntryPatternPath {
        GtpEntryPatternPath::new(path.join(self.relative_path()))
    }
}

impl GtpRelativePath for GtpEntryPatternPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpSrcDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpSrcPathWrapper for GtpEntryPatternPath {
    fn src_path(&self) -> &GtpSrcDirRelativePath {
        &self.0
    }
}

// endregion

// endregion
