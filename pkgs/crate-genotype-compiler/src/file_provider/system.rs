use crate::prelude::internal::*;

pub trait GtcFileProviderSystem {}

impl<Type: GtcFileProviderSystem> GtcFileProvider for Type {
    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn file_write(&self, path: &Path, content: &str) -> Result<()> {
        let parent_dir_path = path
            .parent()
            .ok_or_else(|| miette!("Failed to get parent directory for `{path:?}`"))?;

        fs::create_dir_all(parent_dir_path)
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to create directory `{parent_dir_path:?}`"))?;

        fs::write(path, content)
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to write file `{path:?}`"))?;

        Ok(())
    }
}
