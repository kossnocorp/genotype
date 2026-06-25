use crate::prelude::internal::*;

mod system;
pub use system::*;

/// Project file sink trait. It abstracts the write file system operations for the project.
pub trait GtpFileSink {
    /// Writes a file to the given path.
    fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()>;
}
