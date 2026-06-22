use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleRenderErrorState<ProjectModule: GtlProjectModule> {
    pub source_state: GtlProjectModuleRenderErrorSourceState<ProjectModule>,
    pub error: GtlProjectModuleRenderError,
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerNamed
    for GtlProjectModuleRenderErrorState<ProjectModule>
{
    fn name(&self) -> &'static str {
        "render error"
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleRenderErrorState<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleRenderErrorState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        self.source_state.converted()
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        self.source_state.converted_mut()
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerMaybeResolved
    for GtlProjectModuleRenderErrorState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<Self::ProjectModule>> {
        self.source_state.resolved()
    }
}
