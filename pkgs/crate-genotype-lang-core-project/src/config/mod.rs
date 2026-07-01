use crate::prelude::internal::*;

mod paths;

mod error;
pub use error::*;

pub struct GtlConfig<'project, LangConfig: GtpLangConfig> {
    project: &'project GtProject,
    lang_config: &'project LangConfig,
}

impl<'project, LangConfig: GtpLangConfig> GtlConfig<'project, LangConfig> {
    pub fn new(project: &'project GtProject, lang_config: &'project LangConfig) -> Self {
        GtlConfig {
            project,
            lang_config,
        }
    }

    pub fn lang_config_health_check(&self) -> Vec<GtDiagnostic> {
        self.lang_config
            .health_check(&self.project.paths().config_file, self.package_enabled())
    }

    pub fn project_paths(&self) -> &'project GtpPaths {
        self.project.paths()
    }

    pub fn lang_config(&self) -> &'project LangConfig {
        self.lang_config
    }

    pub fn package_enabled(&self) -> bool {
        self.project.config().lang_package_enabled(self.lang_config)
    }

    pub fn project_name(&self) -> &'project str {
        &self.project.name()
    }

    pub fn project_version(&self) -> Option<&'project Version> {
        self.project.config().version.as_ref()
    }
}
