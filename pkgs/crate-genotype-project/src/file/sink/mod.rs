use crate::prelude::internal::*;

mod system;
pub use system::*;

/// Project file sink trait. It abstracts the write file system operations for the project.
pub trait GtpFileSink<Kind> {
    /// Writes a file to the given path.
    fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()>;
}

pub trait GtpFileSinkProvider<Kind> {
    /// Writes a file to the given path.
    fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()>;
}

impl<Type, Kind> GtpFileSink<Kind> for Type
where
    Type: GtpFileSinkProvider<Kind> + ?Sized,
{
    fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()> {
        GtpFileSinkProvider::write_file(self, path, content)
    }
}
