use crate::prelude::internal::*;

mod os;
pub use os::*;

mod remote;
pub use remote::*;

/// Project file source trait. It abstracts the read file system operations for the project.
#[allow(async_fn_in_trait)]
pub trait GtbFsSource<Kind>: GtbFsEnv {
    /// Globs files from the given path.
    async fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>>;

    /// Reads a file from the given path.
    async fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String>;

    /// Checks if the given path exists.
    async fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Checks if the given path is a file.
    async fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Searches for a file path in the current environment.
    async fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath>;
}
