//! Entry paths. It is the fourth level of paths, relative to [GtpSrcDirPath].
//!
//! Entry is the path pattern that allows to match modules. It is defined in the config and
//! by default equals to `"**/*.type"`.
//!
//! Types:
//!
//! - [GtpEntryPattern]: Entry pattern path relative to [GtpCwdPath].
//! - [GtpSrcDirRelativeEntryPattern]: Entry pattern path relative to [GtpSrcDirPath].

use crate::prelude::internal::*;

// region: Cwd-relative entry pattern

gtp_cwd_relative_path_wrapper_newtype!(
    /// Entry pattern path relative to cwd.
    pub struct GtpEntryPattern(GtpCwdRelativePath);
);

// endregion

// region: Src dir-relative entry pattern

gtp_relative_path_newtype!(
    /// Entry pattern path relative to the source directory.
    pub struct GtpSrcDirRelativeEntryPattern;
    parent: GtpSrcDirPath;
);

impl Default for GtpSrcDirRelativeEntryPattern {
    fn default() -> Self {
        Self::new("**/*.type".into())
    }
}

// endregion
