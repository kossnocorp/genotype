use crate::prelude::internal::*;

mod os;
pub use os::*;

mod remote;
pub use remote::*;

/// Project file sink trait. It abstracts the write file system operations for the project.
#[allow(async_fn_in_trait)]
pub trait GtbFsSink<Kind> {
    /// Writes a file to the given path.
    async fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()>;

    /// Removes a file at the given path.
    async fn remove_file(&self, path: &GtpCwdRelativePath) -> Result<()>;
}
