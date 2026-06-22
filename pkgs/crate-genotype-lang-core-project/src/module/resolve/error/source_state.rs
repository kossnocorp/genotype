use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum GtlProjectModuleResolveErrorSourceState<ProjectModule: GtlProjectModule> {
    Converted(GtlProjectModuleConverted<ProjectModule>),
    Resolved(GtlProjectModuleResolved<ProjectModule>),
    Rendered(GtlProjectModuleRendered<ProjectModule>),
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleResolveErrorSourceState<ProjectModule> {
    pub fn to_error_state(self, error: impl GtlError) -> GtlProjectModuleState<ProjectModule> {
        GtlProjectModuleState::ResolveError(GtlProjectModuleResolveErrorState {
            source_state: self,
            error: GtlProjectModuleResolveError::ResolveError {
                error: Box::new(error),
            },
        })
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleResolveErrorSourceState<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleResolveErrorSourceState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        match &self {
            GtlProjectModuleResolveErrorSourceState::Converted(converted) => converted,
            GtlProjectModuleResolveErrorSourceState::Resolved(resolved) => resolved.converted(),
            GtlProjectModuleResolveErrorSourceState::Rendered(rendered) => rendered.converted(),
        }
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        match self {
            GtlProjectModuleResolveErrorSourceState::Converted(converted) => converted,
            GtlProjectModuleResolveErrorSourceState::Resolved(resolved) => resolved.converted_mut(),
            GtlProjectModuleResolveErrorSourceState::Rendered(rendered) => rendered.converted_mut(),
        }
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerMaybeResolved
    for GtlProjectModuleResolveErrorSourceState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<Self::ProjectModule>> {
        match &self {
            GtlProjectModuleResolveErrorSourceState::Converted(_) => None,
            GtlProjectModuleResolveErrorSourceState::Resolved(resolved) => Some(resolved),
            GtlProjectModuleResolveErrorSourceState::Rendered(rendered) => rendered.resolved(),
        }
    }
}
