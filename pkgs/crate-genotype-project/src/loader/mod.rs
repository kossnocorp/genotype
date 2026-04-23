use crate::prelude::internal::*;

mod parallel;
use miette::Context;
pub use parallel::*;

/// Project loader trait. It defines the interface for loading a project. It bounds to the project
/// source trait to provide file system interop.
pub trait GtpLoader: GtpSource {
    /// Creates a new project.
    fn create_project(&self) -> Result<GtProject> {
        let config_path = self.find_config_path()?;
        let config = self.load_config(&config_path)?;
        let project = GtProject::try_new(config);
        Ok(project)
    }

    /// Loads project config from the given path.
    fn load_config(&self, path: &GtpCwdRelativePath) -> Result<GtpConfig> {
        let config_source = self.read_file(path)?;
        GtpConfig::parse(config_source)
    }

    /// Finds project config path in current environment.
    fn find_config_path(&self) -> Result<GtpCwdRelativePath> {
        let config_path = self
            .find_file("genotype.toml")
            .wrap_err("failed to find config file")?;
        Ok(config_path)
    }

    /// Loads all project modules.
    fn load_all_modules(&self, project: &mut GtProject) -> Result<()>;
}
