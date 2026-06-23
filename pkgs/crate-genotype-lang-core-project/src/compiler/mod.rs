use crate::prelude::internal::*;

mod notices;
use notices::*;

pub trait GtlCompiler<'project>
where
    GtlProjectModuleTypeLangConfig<Self::ProjectModule>: 'project,
{
    type ProjectModule: GtlProjectModule;

    type Manifest<'config>: GtlManifest<'project, 'config, ProjectModule = Self::ProjectModule>
    where
        'project: 'config;

    fn lang(&self) -> GtLang;

    fn project(&self) -> &GtProject;

    fn config(&self) -> &GtlConfig<'project, GtlProjectModuleTypeLangConfig<Self::ProjectModule>>;

    fn new(project: &'project GtProject) -> Self;

    fn lang_config(&self) -> &'project GtlProjectModuleTypeLangConfig<Self::ProjectModule> {
        self.config().lang_config
    }

    fn enabled(&self) -> bool {
        self.lang_config().common().enabled
    }

    fn compile(&self) -> Result<Option<GtlDist>, GtlProjectError> {
        if !self.enabled() {
            return Ok(None);
        }

        let mut notices = vec![];

        notices.extend(self.config().lang_config_health_check());

        let mut lang_project = GtlProject::<Self::ProjectModule>::new(self.config());
        lang_project.convert(&self.project().modules);
        lang_project.resolve()?;
        lang_project.render();

        notices.extend(generate_module_notices(&lang_project.modules));

        let mut dist = GtlDist::new(&lang_project.modules, notices);

        if let Some(package_files) = self.generate_package_files(&lang_project) {
            dist.pack_extra_files(package_files, None);
        }

        if let Some((extra_files, extra_file_notices)) = self.generate_extra_files(&lang_project) {
            dist.pack_extra_files(extra_files, extra_file_notices);
        }

        Ok(Some(dist))
    }

    fn generate_extra_files(
        &self,
        _project: &GtlProject<'project, '_, Self::ProjectModule>,
    ) -> Option<GtlGenerations<Self::ProjectModule>> {
        None
    }

    fn generate_package_files(
        &self,
        project: &GtlProject<'project, '_, Self::ProjectModule>,
    ) -> Option<Vec<GtlGeneration<Self::ProjectModule>>> {
        if !self.config().package_enabled {
            return None;
        }

        let mut files = vec![];

        files.push(self.generate_manifest_file(project));

        if let Some(gitignore) = self.generate_gitignore() {
            files.push(gitignore);
        }

        Some(files)
    }

    fn generate_manifest_file(
        &self,
        project: &GtlProject<'project, '_, Self::ProjectModule>,
    ) -> GtlGeneration<Self::ProjectModule> {
        let manifest = Self::Manifest::<'_>::new(self.config());
        let file = manifest.generate(&project.dependencies());
        file.into()
    }

    fn generate_gitignore(&self) -> Option<GtlGeneration<Self::ProjectModule>> {
        self.gitignore_source_code().map(|source_code| {
            GtlProjectFileExtraGenerated {
                path: self.config().pkg_file_path(&".gitignore".into()),
                source_code,
            }
            .into()
        })
    }

    fn gitignore_source_code(&self) -> Option<String>;
}
