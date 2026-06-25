use crate::prelude::internal::*;

/// System project file access. It is base trait for system project file source and sink traits.
pub trait GtpFileAccessSystem {
    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath;

    /// Resolves full path from the given relative path using the file system.
    fn resolve_path_buf(&self, path: &GtpCwdRelativePath) -> PathBuf {
        path.to_path_buf()
    }
}
