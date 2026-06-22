use crate::prelude::internal::*;

mod paths;

mod error;
pub use error::*;

pub struct GtlConfig<'project, LangConfig: GtpLangConfig> {
    pub project_paths: &'project GtpPaths,
    pub lang_config: &'project LangConfig,
    pub package_enabled: bool,
    pub project_version: Option<&'project Version>,
}

impl<'project, LangConfig: GtpLangConfig> GtlConfig<'project, LangConfig> {
    pub fn new(
        config: &'project GtpConfig,
        project_paths: &'project GtpPaths,
        lang_config: &'project LangConfig,
    ) -> Self {
        GtlConfig {
            project_paths,
            lang_config,
            package_enabled: config.lang_package_enabled(lang_config),
            project_version: config.version.as_ref(),
        }
    }

    pub fn lang_config_health_check(&self) -> Vec<GtNotice> {
        self.lang_config
            .health_check(&self.project_paths.config_file, self.package_enabled)
    }
}
