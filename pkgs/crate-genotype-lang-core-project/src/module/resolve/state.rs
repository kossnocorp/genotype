use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleResolved<ProjectModule: GtlProjectModule> {
    pub converted: GtlProjectModuleConverted<ProjectModule>,
    pub resolved_module: ProjectModule,
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerNamed
    for GtlProjectModuleResolved<ProjectModule>
{
    fn name(&self) -> &'static str {
        "resolved"
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleResolved<ProjectModule> {
    pub fn to_rendered(self, source_code: String) -> GtlProjectModuleRendered<ProjectModule> {
        GtlProjectModuleRendered {
            resolved: self,
            source_code,
        }
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleResolved<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleResolved<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        &self.converted
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        &mut self.converted
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerResolved
    for GtlProjectModuleResolved<ProjectModule>
{
    type Module = ProjectModule;

    fn resolved(&self) -> &GtlProjectModuleResolved<ProjectModule> {
        self
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToResolveError
    for GtlProjectModuleResolved<ProjectModule>
{
    fn to_resolve_error_source_state(
        self,
    ) -> GtlProjectModuleResolveErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleResolveErrorSourceState::Resolved(self)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToRenderError
    for GtlProjectModuleResolved<ProjectModule>
{
    fn to_render_error_source_state(
        self,
    ) -> GtlProjectModuleRenderErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleRenderErrorSourceState::Resolved(self)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectModuleResolved<ProjectModule>>
    for GtlProjectModuleState<ProjectModule>
{
    fn from(val: GtlProjectModuleResolved<ProjectModule>) -> Self {
        GtlProjectModuleState::Resolved(val)
    }
}
