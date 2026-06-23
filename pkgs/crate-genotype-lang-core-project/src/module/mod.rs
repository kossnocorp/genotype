use crate::prelude::internal::*;

mod state;
pub use state::*;

mod convert;
pub use convert::*;

mod resolve;
pub use resolve::*;

mod render;
pub use render::*;

pub type GtlProjectModuleTypeLangConfig<ProjectModule> =
    <ProjectModule as GtlProjectModule>::LangConfig;

pub type GtlProjectModuleTypeModule<'context, ProjectModule> =
    <ProjectModule as GtlProjectModule>::Module<'context>;

pub type GtlProjectModuleTypeDependencyIdent<'context, ProjectModule> =
    GtlModuleTypeDependencyIdent<'context, GtlProjectModuleTypeModule<'context, ProjectModule>>;

pub trait GtlProjectModule: Clone {
    type LangConfig: GtpLangConfig;
    type Module<'context>: GtlModule<'context>;

    fn module(&self) -> &Self::Module<'_>;

    fn convert(
        lang_config: &Self::LangConfig,
        resolved: &GtpModuleResolved,
    ) -> Result<Self, Box<dyn GtlError>>
    where
        Self: Sized;

    /// Resolves the project modules. Some language implementations may choose to override
    /// the function to implement custom resolve logic. The default implementation resolves all
    /// eligible modules. See [[GtlProjectModule::resolve_all_eligible_modules]] for more details.
    fn resolve_modules(
        _lang_config: &Self::LangConfig,
        modules: &mut IndexMap<GtpModulePath, GtlProjectModuleState<Self>>,
    ) -> Result<(), GtlProjectError>
    where
        Self: Sized,
    {
        Self::resolve_all_eligible_modules(modules);
        Ok(())
    }

    /// Resolves all project modules that are in the [[GtlProjectModuleState::Converted]] state to
    /// the [[GtlProjectModuleState::Resolved]] state. The errored and resolved modules are
    /// maintained while the remaining states are converted to the resolve error state.
    fn resolve_all_eligible_modules(
        modules: &mut IndexMap<GtpModulePath, GtlProjectModuleState<Self>>,
    ) where
        Self: Sized,
    {
        *modules = mem::take(modules)
            .into_iter()
            .map(|(module_path, module_state)| {
                let new_state = match module_state {
                    state @ GtlProjectModuleState::ConvertError(_)
                    | state @ GtlProjectModuleState::ResolveError(_)
                    | state @ GtlProjectModuleState::RenderError(_)
                    | state @ GtlProjectModuleState::Resolved(_) => {
                        // Do nothing as it is in the final state.
                        state
                    }

                    GtlProjectModuleState::Rendered(inner) => inner.to_resolve_error_state(),

                    GtlProjectModuleState::Converted(inner) => {
                        let resolved_module = inner.project_module.clone();
                        GtlProjectModuleState::Resolved(GtlProjectModuleResolved {
                            converted: inner,
                            resolved_module,
                        })
                    }
                };
                (module_path, new_state)
            })
            .collect();
    }

    fn render<'context>(
        &'context self,
        lang_config: &'context Self::LangConfig,
    ) -> Result<String, impl GtlError + 'context> {
        let mut context = self.new_render_context(lang_config);
        self.module().render(Default::default(), &mut context)
    }

    fn new_render_context<'config>(
        &self,
        lang_config: &'config Self::LangConfig,
    ) -> GtlModuleTypeRenderContext<'config, Self::Module<'config>>;

    fn global_dependencies(
        _lang_config: &Self::LangConfig,
    ) -> Option<IndexSet<GtlModuleTypeDependencyIdent<'_, Self::Module<'_>>>> {
        None
    }

    fn dependencies(&self) -> IndexSet<GtlProjectModuleTypeDependencyIdent<'_, Self>> {
        self.module().dependencies()
    }
}
