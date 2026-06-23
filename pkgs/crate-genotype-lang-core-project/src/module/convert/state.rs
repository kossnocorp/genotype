use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleConverted<ProjectModule: GtlProjectModule> {
    pub source_path: GtpModulePath,
    pub target_path: GtpTargetFilePath,
    pub project_module: ProjectModule,
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleConverted<ProjectModule> {
    pub fn to_resolved(self) -> GtlProjectModuleResolved<ProjectModule> {
        let resolved_module = self.project_module.clone();
        GtlProjectModuleResolved {
            converted: self,
            resolved_module,
        }
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerNamed
    for GtlProjectModuleConverted<ProjectModule>
{
    fn name(&self) -> &'static str {
        "converted"
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInner
    for GtlProjectModuleConverted<ProjectModule>
{
    fn source_path(&self) -> &GtpModulePath {
        &self.source_path
    }

    fn target_path(&self) -> Option<&GtpTargetFilePath> {
        Some(&self.target_path)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleConverted<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        self
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<ProjectModule> {
        self
    }

    fn project_module(&self) -> &ProjectModule {
        &self.project_module
    }

    fn project_module_mut(&mut self) -> &mut ProjectModule {
        &mut self.project_module
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectModuleConverted<ProjectModule>>
    for GtlProjectModuleState<ProjectModule>
{
    fn from(val: GtlProjectModuleConverted<ProjectModule>) -> Self {
        GtlProjectModuleState::Converted(val)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToResolveError
    for GtlProjectModuleConverted<ProjectModule>
{
    fn to_resolve_error_source_state(
        self,
    ) -> GtlProjectModuleResolveErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleResolveErrorSourceState::Converted(self)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToRenderError
    for GtlProjectModuleConverted<ProjectModule>
{
    fn to_render_error_source_state(
        self,
    ) -> GtlProjectModuleRenderErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleRenderErrorSourceState::Converted(self)
    }
}
