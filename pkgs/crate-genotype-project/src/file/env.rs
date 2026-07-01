use crate::prelude::internal::*;

pub trait GtpFileEnv {
    /// Returns the current working directory path.
    fn cwd_path(&self) -> &GtpCwdPath;

    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath;
}
