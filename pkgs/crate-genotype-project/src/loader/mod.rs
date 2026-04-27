use crate::prelude::internal::*;

// region: Modules

mod parallel;
pub use parallel::*;

// endregion

// region Loader trait

/// Project loader trait. It defines the interface for loading a project. It bounds to the project
/// source trait to provide file system interop.
pub trait GtpLoader<ProjectRef>: GtpSource {
    /// Creates a new project.
    fn create_project(
        &self,
        config_path: Option<&GtpCwdRelativeOrAbsoluteStringPath>,
    ) -> Result<GtProject> {
        let config_file_path = self.find_config_path(config_path)?;
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
    fn find_config_path(
        &self,
        config_path: Option<&GtpCwdRelativeOrAbsoluteStringPath>,
    ) -> Result<GtpConfigFilePath> {
        match config_path {
            Some(config_path) => {
                let config_path = config_path
                    .try_into()
                    .wrap_err_with(|| format!("failed to normalize base path '{config_path}'"))?;
                if self.is_file(&config_path) {
                    Ok(config_path.into())
                } else {
                    Err(miette!("config file '{config_path}' does not exist"))
                }
            }

            None => {
                let config_path = self
                    .find_file("genotype.toml")
                    .wrap_err("failed to find config file")?;
                Ok(config_path.into())
            }
        }
    }

    /// Loads all project modules.
    fn load_all_modules(&self, project: GtProject) -> Result<GtProject> {
        let module_entries = self
            .glob(project.paths.entry.as_ref())?
            .into_iter()
            .map(|path| path.into())
            .collect::<Vec<GtpModulePath>>();

        ensure!(
            module_entries.len() > 0,
            "no module files found for entry pattern '{}'",
            project.paths.entry.display()
        );

        let mut project = self.load_module_entries(project, module_entries)?;

        project.resolve_modules()?;
        project.sort_modules();

        Ok(project)
    }

    /// Loads module entries.
    fn load_module_entries(
        &self,
        project: GtProject,
        module_entries: Vec<GtpModulePath>,
    ) -> Result<GtProject>;

    /// Parse module source code.
    fn parse_module(
        &self,
        path: &GtpModulePath,
        module_id_result: Result<Option<GtModuleId>>,
    ) -> Result<Option<GtpModuleParse>, GtpModuleError> {
        let module_id = module_id_result.map_err(|err| GtpModuleError::Init {
            path: path.clone(),
            message: err.to_string(),
        })?;

        let parse = match module_id {
            Some(module_id) => {
                let source_code = self.read_file(path.cwd_relative_path()).map_err(|err| {
                    GtpModuleError::Read {
                        path: path.clone(),
                        message: err.to_string(),
                    }
                })?;

                let parse = GtpModule::parse(&path, module_id, source_code)?;
                Some(parse)
            }

            None => None,
        };

        Ok(parse)
    }

    /// Loads a project module. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn load_project_module<'a>(
        &'a self,
        project: &ProjectRef,
        path: GtpModulePath,
    ) -> Result<Option<Vec<GtpModulePath>>> {
        let module_id_result = self.init_project_module(&project, &path);
        let parse_result = self.parse_module(&path, module_id_result);

        let dep_paths = match parse_result {
            Ok(Some(module_state)) => {
                let dep_paths = module_state.dep_paths();
                self.set_project_module(&project, &path, module_state.into())?;
                Some(dep_paths)
            }

            Ok(None) => {
                // Module already initialized, skip loading.
                None
            }

            Err(err) => {
                self.set_project_module(&project, &path, GtpModule::Error(err))?;
                None
            }
        };

        Ok(dep_paths)
    }

    /// Initializes the module. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn init_project_module<'a>(
        &'a self,
        project: &ProjectRef,
        path: &GtpModulePath,
    ) -> Result<Option<GtModuleId>>;

    /// Sets the module state. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn set_project_module(
        &self,
        project: &ProjectRef,
        path: &GtpModulePath,
        state: GtpModule,
    ) -> Result<()>;
}

// endregion
