use crate::prelude::internal::*;

// region: Modules

mod parallel;
pub use parallel::*;

mod serial;
pub use serial::*;

// endregion

// region Loader trait

/// Project loader trait. It defines the interface for loading a project. It bounds to the project
/// source trait to provide file system interop.
pub trait GtpLoader<ProjectRef>: GtpFileSource {
    /// Creates a new project.
    fn create_project(&self, config_path: Option<&GtpCwdRelativePath>) -> Result<GtProject> {
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
        config_path: Option<&GtpCwdRelativePath>,
    ) -> Result<GtpConfigFilePath> {
        match config_path {
            Some(config_path) => {
                if self.is_file(config_path)? {
                    Ok(config_path.clone().into())
                } else {
                    Err(miette!("Config file '{config_path}' does not exist"))
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
            .glob_files(project.paths.entry.as_ref())?
            .into_iter()
            .map(|file_path| GtpModulePath::from_cwd_relative_path(file_path))
            .collect::<Vec<GtpModulePath>>();

        ensure!(
            !module_entries.is_empty(),
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
        source: &GtpModuleSource,
        module_id_result: Result<Option<GtModuleId>>,
    ) -> Result<Option<GtpModuleParse>, GtpModuleError> {
        let path = source.path();
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

                let parse = GtpModule::parse(path, source, module_id, source_code)?;
                Some(parse)
            }

            None => None,
        };

        Ok(parse)
    }

    /// Loads a project module. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn load_project_module(
        &self,
        project: &ProjectRef,
        source: &GtpModuleSource,
    ) -> Result<Option<Vec<GtpModuleSource>>> {
        self.add_project_module_source(project, source)?;

        let module_id_result = self.init_project_module(project, source);
        let parse_result = self.parse_module(source, module_id_result);

        let module_deps = match parse_result {
            Ok(Some(module_state)) => {
                let module_deps = module_state.deps();
                self.set_project_module(project, source, module_state.into())?;
                Some(module_deps)
            }

            Ok(None) => {
                // Module already initialized, skip loading.
                None
            }

            Err(err) => {
                self.set_project_module(project, source, GtpModule::Error(source.clone(), err))?;
                None
            }
        };

        Ok(module_deps)
    }

    /// Initializes the module. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn init_project_module(
        &self,
        project: &ProjectRef,
        module: &GtpModuleSource,
    ) -> Result<Option<GtModuleId>>;

    /// Sets the module state. It relies on project ref allowing to implement runtime-specific
    /// project reference counting and mutability.
    fn set_project_module(
        &self,
        project: &ProjectRef,
        module: &GtpModuleSource,
        state: GtpModule,
    ) -> Result<()>;

    /// Adds module source to the project. It provides map of all module references.
    fn add_project_module_source(
        &self,
        project: &ProjectRef,
        source: &GtpModuleSource,
    ) -> Result<()>;
}

// endregion
