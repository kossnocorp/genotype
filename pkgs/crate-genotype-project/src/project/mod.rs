use crate::prelude::internal::*;

// region: Modules

mod resolve;
pub use resolve::*;

mod diagnostics;

mod pkg;

mod sources;

// endregion

pub const DEFAULT_PROJECT_NAME: &str = "types";

/// Genotype project. Represents configuration with currently loaded modules.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtProject {
    /// Known project modules mapped by their workspace path.
    modules: IndexMap<GtpModulePath, GtpModule>,

    /// Known module sources.
    // TODO: It must rebuild when a module is changed.
    module_sources: IndexMap<GtpModulePath, IndexSet<GtpModuleSource>>,

    /// Project name resolved from config, config parent directory, or default.
    name: String,

    /// Project configuration.
    config: GtpConfig,

    /// Project paths.
    paths: GtpPaths,
}

impl GtProject {
    pub fn try_new(
        fallback_name: String,
        config_file_path: GtpConfigFilePath,
        config: GtpConfig,
    ) -> Result<Self> {
        let paths = GtProject::try_new_paths(config_file_path, &config)
            .wrap_err("failed to initialize project paths from config")?;
        let name = config.name.clone().unwrap_or(fallback_name);

        Ok(Self {
            modules: IndexMap::new(),
            module_sources: IndexMap::new(),
            name,
            config,
            paths,
        })
    }

    fn try_new_paths(config_file_path: GtpConfigFilePath, config: &GtpConfig) -> Result<GtpPaths> {
        let config_dir = config_file_path.to_config_dir_path();
        let root = config.root.to_cwd_relative_path(&config_dir).into();
        let dist = config.dist.to_cwd_relative_path(&root).into();
        let src = config.src.to_cwd_relative_path(&root).into();
        let entry = config.entry.to_cwd_relative_path(&src).into();

        Ok(GtpPaths {
            config_file: config_file_path,
            root,
            dist,
            src,
            entry,
        })
    }

    pub fn modules(&self) -> &IndexMap<GtpModulePath, GtpModule> {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut IndexMap<GtpModulePath, GtpModule> {
        &mut self.modules
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> &GtpConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut GtpConfig {
        &mut self.config
    }

    pub fn paths(&self) -> &GtpPaths {
        &self.paths
    }

    /// Tries to initialize a module in the project. If the module already initialized, it resolves
    /// none signifying that the module is already processing or loaded. Otherwise, it initializes
    /// the module and returns some [GtModuleId].
    pub fn init_module(&mut self, source: &GtpModuleSource) -> Result<Option<GtModuleId>> {
        let path = source.path();
        match self.has_module(path) {
            true => Ok(None),
            false => {
                self.modules
                    .insert(path.clone(), GtpModule::Initialized(source.clone()));
                let module_id = path.to_module_id(&self.paths.src)?;
                Ok(Some(module_id))
            }
        }
    }

    /// Checks if the module is already initialized in the project.
    fn has_module(&self, path: &GtpModulePath) -> bool {
        self.modules.contains_key(path)
    }

    /// Sets the state of a module in the project.
    pub fn set_module(&mut self, path: &GtpModulePath, module_state: GtpModule) {
        self.modules.insert(path.clone(), module_state);
    }

    pub fn lang_enabled(&self, lang: GtLang) -> bool {
        self.config.lang_enabled(lang)
    }

    pub fn lang_config(&self, lang: GtLang) -> &dyn GtpLangConfig {
        self.config.lang(lang)
    }

    pub fn lang_package_enabled(&self, lang_config: &dyn GtpLangConfig) -> bool {
        self.config.lang_package_enabled(lang_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uses_config_name() {
        let config = GtpConfig::parse(r#"name = "example-package""#.into()).unwrap();
        let project =
            GtProject::try_new("fallback-name".into(), "genotype.toml".into(), config).unwrap();
        assert_equal!(project.name, "example-package");
    }

    #[test]
    fn test_uses_fallback_name() {
        let project = GtProject::try_new(
            "fallback-name".into(),
            "genotype.toml".into(),
            GtpConfig::default(),
        )
        .unwrap();
        assert_equal!(project.name, "fallback-name");
    }
}
