use crate::prelude::internal::*;

mod system;
pub use system::*;

/// Project file source trait. It abstracts the read file system operations for the project.
pub trait GtpFileSource<Kind>: GtpFileEnv {
    /// Globs files from the given path.
    fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>>;

    /// Reads a file from the given path.
    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String>;

    /// Checks if the given path exists.
    fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Checks if the given path is a file.
    fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Searches for a file path in the current environment.
    fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath>;
}

pub trait GtpFileSourceProvider<Kind> {
    /// Globs files from the given path.
    fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>>;

    /// Reads a file from the given path.
    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String>;

    /// Checks if the given path exists.
    fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Checks if the given path is a file.
    fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool>;

    /// Searches for a file path in the current environment.
    fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath>;
}

impl<Type: GtpFileEnv, Kind> GtpFileSource<Kind> for Type
where
    Type: GtpFileSourceProvider<Kind> + ?Sized,
{
    fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>> {
        GtpFileSourceProvider::glob_files(self, path)
    }

    fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String> {
        GtpFileSourceProvider::read_file(self, path)
    }

    fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        GtpFileSourceProvider::file_exists(self, path)
    }

    fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        GtpFileSourceProvider::is_file(self, path)
    }

    fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath> {
        GtpFileSourceProvider::find_file(self, file_name)
    }
}
