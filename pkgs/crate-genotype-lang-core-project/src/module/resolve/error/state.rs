use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleResolveErrorState<ProjectModule: GtlProjectModule> {
    pub source_state: GtlProjectModuleResolveErrorSourceState<ProjectModule>,
    pub error: GtlProjectModuleResolveError,
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerNamed
    for GtlProjectModuleResolveErrorState<ProjectModule>
{
    fn name(&self) -> &'static str {
        "resolve error"
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleResolveErrorState<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleResolveErrorState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        self.source_state.converted()
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        self.source_state.converted_mut()
    }
}
