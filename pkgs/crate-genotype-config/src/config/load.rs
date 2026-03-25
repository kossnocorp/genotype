use crate::prelude::internal::*;

impl GtConfig {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let file = Self::find(path)?;

        let config_parent = if let Some(parent) = file.parent() {
            RelativePathBuf::from_path(parent)
        } else {
            RelativePathBuf::from_path(".")
        }
        .into_diagnostic()?;

        let config_toml_str = fs::read_to_string(file).into_diagnostic()?;
        let mut config: GtConfig = Self::from_toml_str(&config_toml_str)?;
        config.root = GtRootPath::new(config_parent.join_normalized(config.root.relative_path()));

        Ok(config)
    }

    pub fn find(path: &PathBuf) -> Result<PathBuf> {
        let mut current = if path.is_dir() {
            Some(path.as_path())
        } else {
            path.parent()
        };

        while let Some(dir) = current {
            let file = dir.join(GTCONFIG_FILE);
            if file.is_file() {
                return Ok(file);
            }
            current = dir.parent();
        }

        Err(GtConfigError::MissingConfig(path.clone())).into_diagnostic()
    }
}
