use crate::prelude::internal::*;

mod parallel;
use miette::Context;
pub use parallel::*;

/// Project loader trait. It defines the interface for loading a project. It bounds to the project
/// source trait to provide file system interop.
pub trait GtpLoader: GtpSource {
    /// Creates a new project.
    fn create_project(&self) -> Result<GtProject> {
        let config_file_path = self.find_config_path()?;
        let config = self.load_config(&config_file_path)?;
        let project = GtProject::try_new(config_file_path, config)?;
        Ok(project)
    }

    /// Loads project config from the given path.
    fn load_config(&self, path: &GtpConfigFilePath) -> Result<GtpConfig> {
        let config_source = self.read_file(path.as_ref())?;
        let config = GtpConfig::parse(config_source)?;
        Ok(config)
    }

    /// Finds project config path in current environment.
    fn find_config_path(&self) -> Result<GtpConfigFilePath> {
        let config_path = self
            .find_file("genotype.toml")
            .wrap_err("failed to find config file")?;
        Ok(config_path.into())
    }

    /// Loads all project modules.
    fn load_all_modules(&self, project: &mut GtProject) -> Result<()>;
}
