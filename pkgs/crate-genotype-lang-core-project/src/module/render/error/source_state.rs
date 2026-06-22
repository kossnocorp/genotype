use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum GtlProjectModuleRenderErrorSourceState<ProjectModule: GtlProjectModule> {
    Converted(GtlProjectModuleConverted<ProjectModule>),
    Resolved(GtlProjectModuleResolved<ProjectModule>),
    Rendered(GtlProjectModuleRendered<ProjectModule>),
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleRenderErrorSourceState<ProjectModule> {
    pub fn to_error_state(self, error: impl GtlError) -> GtlProjectModuleState<ProjectModule> {
        GtlProjectModuleState::RenderError(GtlProjectModuleRenderErrorState {
            source_state: self,
            error: GtlProjectModuleRenderError::RenderError {
                error: Box::new(error),
            },
        })
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConvertedWrapper
    for GtlProjectModuleRenderErrorSourceState<ProjectModule>
{
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerConverted
    for GtlProjectModuleRenderErrorSourceState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<ProjectModule> {
        match &self {
            GtlProjectModuleRenderErrorSourceState::Converted(converted) => converted,
            GtlProjectModuleRenderErrorSourceState::Resolved(resolved) => resolved.converted(),
            GtlProjectModuleRenderErrorSourceState::Rendered(rendered) => rendered.converted(),
        }
    }

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule> {
        match self {
            GtlProjectModuleRenderErrorSourceState::Converted(converted) => converted,
            GtlProjectModuleRenderErrorSourceState::Resolved(resolved) => resolved.converted_mut(),
            GtlProjectModuleRenderErrorSourceState::Rendered(rendered) => rendered.converted_mut(),
        }
    }
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleStateInnerMaybeResolved
    for GtlProjectModuleRenderErrorSourceState<ProjectModule>
{
    type ProjectModule = ProjectModule;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<Self::ProjectModule>> {
        match &self {
            GtlProjectModuleRenderErrorSourceState::Converted(_) => None,
            GtlProjectModuleRenderErrorSourceState::Resolved(resolved) => Some(resolved),
            GtlProjectModuleRenderErrorSourceState::Rendered(rendered) => rendered.resolved(),
        }
    }
}
