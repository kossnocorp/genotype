use crate::prelude::internal::*;

/// System project file sink. It provides file system interop for the project. It is the default
/// system project sink used by the system project runtime.
pub trait GtpFileSinkSystem: GtpFileAccessSystem {}

impl<Type: GtpFileSinkSystem + ?Sized> GtpFileSink for Type {
    /// Writes a file to the given path using the file system.
    fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()> {
        let path = self.resolve_path_buf(path);

        let parent_dir_path = path
            .parent()
            .ok_or_else(|| miette!("Failed to get parent directory for `{path:?}`"))?;

        fs::create_dir_all(parent_dir_path)
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to create directory `{parent_dir_path:?}`"))?;

        fs::write(&path, content)
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to write file `{path:?}`"))?;

        Ok(())
    }
}
