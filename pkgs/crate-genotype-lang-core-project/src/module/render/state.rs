use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleRendered<ProjectModule: GtlProjectModule> {
    pub resolved: GtlProjectModuleResolved<ProjectModule>,
    pub source_code: String,
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerNamed
    for GtlProjectModuleRendered<ProjectModule>
{
    fn name(&self) -> &'static str {
        "rendered"
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleRendered<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleRendered<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        self.resolved.converted()
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        self.resolved.converted_mut()
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerMaybeResolved
    for GtlProjectModuleRendered<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<ProjectModule>> {
        Some(&self.resolved)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToResolveError
    for GtlProjectModuleRendered<ProjectModule>
{
    fn to_resolve_error_source_state(
        self,
    ) -> GtlProjectModuleResolveErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleResolveErrorSourceState::Rendered(self)
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerToRenderError
    for GtlProjectModuleRendered<ProjectModule>
{
    fn to_render_error_source_state(
        self,
    ) -> GtlProjectModuleRenderErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    > {
        GtlProjectModuleRenderErrorSourceState::Rendered(self)
    }
}
