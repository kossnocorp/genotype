use crate::prelude::internal::*;

mod fs;
pub use fs::*;

/// Project source trait. It abstracts the file system operations for the project loader.
pub trait GtpSource {
    /// Globs files from the given path.
    fn glob(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpModulePath>>;

    /// Reads a file from the given path.
    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String>;

    /// Checks if the given path is a file.
    fn is_file(&self, path: &GtpCwdRelativePath) -> bool;

    /// Searches for a file path in the current environment.
    fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath>;
}
