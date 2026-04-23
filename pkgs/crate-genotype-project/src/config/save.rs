use crate::prelude::internal::*;

impl GtpConfig {
    pub fn save(&self, path: &Path) -> Result<()> {
        let file = if path.is_dir() {
            unimplemented!()
            // match Self::find(path) {
            //     Ok(file) => file,
            //     Err(_) => path.join(GTCONFIG_FILE),
            // }
        } else if path.ends_with(GTCONFIG_FILE) {
            path.to_path_buf()
        } else {
            unimplemented!()
            // Self::find(path)?
        };

        fs::write(&file, self.to_toml_str_pruned()?)
            .map_err(|_| GtpConfigError::FailedSaveConfig(file.display().to_string()))
            .into_diagnostic()?;

        Ok(())
    }
}
