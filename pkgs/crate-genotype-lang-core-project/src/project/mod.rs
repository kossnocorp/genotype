use crate::prelude::internal::*;

mod converter;
pub use converter::*;

mod resolver;
pub use resolver::*;

mod renderer;
pub use renderer::*;

pub type GtlProjectModules<ProjectModule> =
    IndexMap<GtpModulePath, GtlProjectModuleState<ProjectModule>>;

pub struct GtlProject<'project, 'config, ProjectModule: GtlProjectModule>
where
    GtlProjectModuleTypeLangConfig<ProjectModule>: 'project,
{
    pub config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<ProjectModule>>,
    pub modules: GtlProjectModules<ProjectModule>,
}

impl<'project, 'config, ProjectModule: GtlProjectModule>
    GtlProject<'project, 'config, ProjectModule>
{
    pub fn new(
        config: &'config GtlConfig<'project, GtlProjectModuleTypeLangConfig<ProjectModule>>,
    ) -> Self {
        let modules = IndexMap::new();
        GtlProject { config, modules }
    }

    pub fn dependencies(&self) -> IndexSet<GtlProjectModuleTypeDependencyIdent<'_, ProjectModule>> {
        let mut dependencies = IndexSet::new();

        for (_, module) in self.modules.iter() {
            if let Some(module_dependencies) = module.dependencies() {
                dependencies.extend(module_dependencies.iter().cloned());
            }
        }

        if let Some(global_dependencies) =
            ProjectModule::global_dependencies(self.config.lang_config)
        {
            dependencies.extend(global_dependencies);
        }

        dependencies
    }
}
