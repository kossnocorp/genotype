use crate::prelude::internal::*;

use std::cell::RefCell;

mod system;
pub use system::*;

mod remote;
pub use remote::*;

#[allow(async_fn_in_trait)]
pub trait GtBackend:
    GtbFsEnv
    + GtbFsSource<<Self as GtBackend>::FileSourceKind>
    + GtbFsSink<<Self as GtBackend>::FileSinkKind>
    + GtbDiagnosticSink<<Self as GtBackend>::DiagnosticSinkKind>
    + GtbFormatterRunner<
        <Self as GtBackend>::FormatterRunnerKind,
        <Self as GtBackend>::DiagnosticSinkKind,
    >
{
    type FileSourceKind;

    type FileSinkKind;

    type DiagnosticSinkKind;

    type FormatterRunnerKind;

    async fn create_project(&self, config_path: Option<&GtpCwdRelativePath>) -> Result<GtProject> {
        create_project(self, config_path).await
    }

    async fn save_config(&self, path: &GtpConfigFilePath, config: &GtpConfig) -> Result<()> {
        let source = config.to_toml_str_pruned()?;
        self.write_file(path.as_ref(), &source).await
    }

    async fn load_all_modules(&self, project: GtProject) -> Result<GtProject> {
        load_all_modules(self, project).await
    }

    async fn create_project_and_load_all_modules(
        &self,
        config_path: Option<&GtpCwdRelativePath>,
    ) -> Result<GtProject> {
        create_project_and_load_all_modules(self, config_path).await
    }
}

async fn create_project_and_load_all_modules<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    config_path: Option<&GtpCwdRelativePath>,
) -> Result<GtProject> {
    let project = create_project(backend, config_path)
        .await
        .wrap_err("Failed to create project")?;

    let project = load_all_modules(backend, project)
        .await
        .wrap_err("Failed to load all project modules")?;

    Ok(project)
}

async fn create_project<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    config_path: Option<&GtpCwdRelativePath>,
) -> Result<GtProject> {
    let config_file_path = find_config_path(backend, config_path).await?;
    let config = load_config(backend, &config_file_path).await?;
    let fallback_name = resolve_fallback_name(backend.cwd_path(), &config_file_path);
    let project = GtProject::try_new(fallback_name, config_file_path, config)?;
    Ok(project)
}

async fn load_config<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    path: &GtpConfigFilePath,
) -> Result<GtpConfig> {
    let config_source_code_content = backend.read_file(path.as_ref()).await?;
    let config_source_code = GtpSourceCode::new(config_source_code_content);
    let config = GtpConfig::from_source_code(config_source_code)?;
    Ok(config)
}

async fn find_config_path<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    config_path: Option<&GtpCwdRelativePath>,
) -> Result<GtpConfigFilePath> {
    match config_path {
        Some(config_path) => {
            if backend.is_file(config_path).await? {
                Ok(config_path.clone().into())
            } else {
                Err(miette!("Config file '{config_path}' does not exist"))
            }
        }

        None => {
            let config_path = backend
                .find_file("genotype.toml")
                .await
                .wrap_err("Failed to find config file")?;
            Ok(config_path.into())
        }
    }
}

async fn load_all_modules<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    project: GtProject,
) -> Result<GtProject> {
    let module_entries = backend
        .glob_files(project.paths().entry.as_ref())
        .await?
        .into_iter()
        .map(GtpModulePath::from_cwd_relative_path)
        .collect::<Vec<GtpModulePath>>();

    ensure!(
        !module_entries.is_empty(),
        "No module files found for entry pattern '{}'",
        project.paths().entry.display()
    );

    let mut project = load_module_entries(backend, project, module_entries).await?;

    project.resolve_modules()?;
    project.type_check_modules()?;
    project.sort_modules();

    Ok(project)
}

async fn load_module_entries<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    project: GtProject,
    module_entries: Vec<GtpModulePath>,
) -> Result<GtProject> {
    let project = RefCell::new(project);
    let entry_module_paths = module_entries.clone();
    for entry_module_path in &module_entries {
        add_project_module_source(&project, &entry_module_path.clone().into())?;
    }

    for entry_module_path in module_entries {
        load_module_recursive(backend, &project, &entry_module_paths, entry_module_path).await?;
    }
    Ok(project.into_inner())
}

fn load_module_recursive<'a, Backend, LoaderModule>(
    backend: &'a Backend,
    project: &'a RefCell<GtProject>,
    entry_module_paths: &'a [GtpModulePath],
    module: LoaderModule,
) -> LocalBoxFuture<'a, Result<()>>
where
    Backend: GtBackend + ?Sized,
    LoaderModule: Into<GtpModuleSource> + 'static,
{
    let module = module.into();
    async move {
        if let Some(dep_paths) = load_project_module(backend, project, &module).await? {
            for dep_path in dep_paths {
                if entry_module_paths.contains(dep_path.path()) {
                    add_project_module_source(project, &dep_path)?;
                    continue;
                }

                load_module_recursive(backend, project, entry_module_paths, dep_path).await?;
            }
        }
        Ok(())
    }
    .boxed_local()
}

async fn parse_module<Backend: GtBackend + ?Sized>(
    backend: &Backend,
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
            let source_code_content =
                backend
                    .read_file(path.cwd_relative_path())
                    .await
                    .map_err(|err| GtpModuleError::Read {
                        path: path.clone(),
                        message: err.to_string(),
                    })?;
            let source_code = GtpSourceCode::new(source_code_content);

            let parse = GtpModule::parse(path, source, module_id, source_code)?;
            Some(parse)
        }

        None => None,
    };

    Ok(parse)
}

async fn load_project_module<Backend: GtBackend + ?Sized>(
    backend: &Backend,
    project: &RefCell<GtProject>,
    source: &GtpModuleSource,
) -> Result<Option<Vec<GtpModuleSource>>> {
    add_project_module_source(project, source)?;

    let module_id_result = init_project_module(project, source);
    let parse_result = parse_module(backend, source, module_id_result).await;

    let module_deps = match parse_result {
        Ok(Some(module_state)) => {
            let module_deps = module_state.deps();
            set_project_module(project, source, module_state.into())?;
            Some(module_deps)
        }

        Ok(None) => None,

        Err(err) => {
            set_project_module(project, source, GtpModule::Error(source.clone(), err))?;
            None
        }
    };

    Ok(module_deps)
}

fn init_project_module(
    project: &RefCell<GtProject>,
    source: &GtpModuleSource,
) -> Result<Option<GtModuleId>> {
    let mut project = project.borrow_mut();
    project.init_module(source)
}

fn set_project_module(
    project: &RefCell<GtProject>,
    source: &GtpModuleSource,
    state: GtpModule,
) -> Result<()> {
    let mut project = project.borrow_mut();
    project.set_module(source.path(), state);
    Ok(())
}

fn add_project_module_source(project: &RefCell<GtProject>, source: &GtpModuleSource) -> Result<()> {
    let mut project = project.borrow_mut();
    project.add_module_source(source.clone());
    Ok(())
}

/// Resolves fallback name for the project. It is used when the project name is not specified
/// in the config file.
fn resolve_fallback_name(cwd_path: &GtpCwdPath, config_file_path: &GtpConfigFilePath) -> String {
    let config_dir_path = config_file_path.to_config_dir_path().to_path_buf();
    dir_name(config_dir_path)
        .or_else(|| dir_name(cwd_path.as_path()))
        .unwrap_or_else(|| DEFAULT_PROJECT_NAME.to_string())
}

fn dir_name<Pth: AsRef<Path>>(path: Pth) -> Option<String> {
    let path = path.as_ref();
    path.file_name()
        .and_then(|dir_name| dir_name.to_str().map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolves_config_dir_name() {
        let config_file_path = GtpConfigFilePath::from_str("dir/project-name/genotype.toml");
        let cwd_path = GtpCwdPath::from("/absolute/path/to/project-dir-name");
        let fallback_name = resolve_fallback_name(&cwd_path, &config_file_path);
        assert_equal!(fallback_name, "project-name");
    }

    #[test]
    fn test_resolves_cwd_dir_name() {
        let config_file_path = GtpConfigFilePath::from_str("genotype.toml");
        let cwd_path = GtpCwdPath::from("/absolute/path/to/project-dir-name");
        let fallback_name = resolve_fallback_name(&cwd_path, &config_file_path);
        assert_equal!(fallback_name, "project-dir-name");
    }

    #[test]
    fn test_resolves_default_project_name() {
        let config_file_path = GtpConfigFilePath::from_str("genotype.toml");
        let cwd_path = GtpCwdPath::from("/");
        let fallback_name = resolve_fallback_name(&cwd_path, &config_file_path);
        assert_equal!(fallback_name, DEFAULT_PROJECT_NAME);
    }
}
