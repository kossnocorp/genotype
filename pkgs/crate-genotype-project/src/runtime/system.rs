use miette::Context;
use relative_path::PathExt;

pub use crate::prelude::internal::*;

/// System project runtime. It combines parallel project loader with file system project source.
/// It is the default project runtime used by the CLI.
pub struct GtpRuntimeSystem {
    /// Base path for the project source to resolve relative file paths.
    base_path: GtpCwdRelativePath,
}

impl GtpRuntimeSystem {
    /// Creates a new system project runtime with the given base path.
    pub fn new(path: &PathBuf) -> Result<Self> {
        let cwd_path = GtpCwdPath::try_new()?;
        println!(">>>>>>>>>>>> cwd path {:?}", cwd_path.as_path());
        let rel_base_path = if path.is_absolute() {
            path.relative_to(&cwd_path.as_path())
                .map_err(|e| miette!(e)).wrap_err_with(||format!("failed to resolve base path from '{}' relative to current working directory '{}'", path.display(), cwd_path.as_path().display()))?
        } else {
            RelativePathBuf::from_path(path)
                .map_err(|e| miette!(e))
                .wrap_err_with(|| {
                    format!(
                        "failed to convert base path '{}' to relative path",
                        path.display()
                    )
                })?
        };
        let base_path = GtpCwdRelativePath::new(rel_base_path);
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
