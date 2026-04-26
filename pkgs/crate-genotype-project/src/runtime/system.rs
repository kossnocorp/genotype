pub use crate::prelude::internal::*;

/// System project runtime. It combines parallel project loader with file system project source.
/// It is the default project runtime used by the CLI.
pub struct GtpRuntimeSystem {
    /// Base path for the project source to resolve relative file paths.
    base_path: GtpCwdRelativePath,
}

impl GtpRuntimeSystem {
    /// Creates a new system project runtime with the given base path.
    pub fn new(path: &GtpCwdRelativeOrAbsoluteStringPath) -> Result<Self> {
        let base_path = path
            .try_into()
            .wrap_err("failed to convert path into a string")?;
        Ok(Self { base_path })
    }
}

impl GtpLoaderParallel for GtpRuntimeSystem {}

impl GtpSourceFs for GtpRuntimeSystem {
    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath {
        &self.base_path
    }
}
