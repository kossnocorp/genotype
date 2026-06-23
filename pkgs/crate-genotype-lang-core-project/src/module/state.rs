use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum GtlProjectModuleState<ProjectModule: GtlProjectModule> {
    ConvertError(GtlProjectModuleConvertErrorState),
    Converted(GtlProjectModuleConverted<ProjectModule>),
    ResolveError(GtlProjectModuleResolveErrorState<ProjectModule>),
    Resolved(GtlProjectModuleResolved<ProjectModule>),
    Rendered(GtlProjectModuleRendered<ProjectModule>),
    RenderError(GtlProjectModuleRenderErrorState<ProjectModule>),
}

impl<ProjectModule: GtlProjectModule> GtlProjectModuleState<ProjectModule> {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ConvertError(inner) => inner.name(),
            Self::Converted(inner) => inner.name(),
            Self::ResolveError(inner) => inner.name(),
            Self::Resolved(inner) => inner.name(),
            Self::RenderError(inner) => inner.name(),
            Self::Rendered(inner) => inner.name(),
        }
    }

    pub fn action(&self) -> &'static str {
        match self {
            Self::ConvertError(_) | Self::Converted(_) => "convert",
            Self::ResolveError(_) | Self::Resolved(_) => "resolve",
            Self::RenderError(_) | Self::Rendered(_) => "render",
        }
    }

    pub fn target_path(&self) -> Option<&GtpTargetFilePath> {
        self.as_inner().target_path()
    }

    pub fn source_path(&self) -> &GtpModulePath {
        self.as_inner().source_path()
    }

    pub fn project_module(&self) -> Option<&ProjectModule> {
        self.converted().map(|converted| converted.project_module())
    }

    pub fn project_module_mut(&mut self) -> Option<&mut ProjectModule> {
        self.converted_mut()
            .map(|converted| converted.project_module_mut())
    }

    pub fn converted(&self) -> Option<&GtlProjectModuleConverted<ProjectModule>> {
        self.as_inner_converted().map(|inner| inner.converted())
    }

    pub fn converted_mut(&mut self) -> Option<&mut GtlProjectModuleConverted<ProjectModule>> {
        self.as_inner_converted_mut()
            .map(|inner| inner.converted_mut())
    }

    pub fn dependencies(
        &self,
    ) -> Option<IndexSet<GtlProjectModuleTypeDependencyIdent<'_, ProjectModule>>> {
        self.converted().map(|generated| generated.dependencies())
    }

    pub fn as_inner(&self) -> &dyn GtlProjectModuleStateInner {
        match self {
            Self::ConvertError(inner) => inner,
            Self::Converted(inner) => inner,
            Self::ResolveError(inner) => inner,
            Self::Resolved(inner) => inner,
            Self::Rendered(inner) => inner,
            Self::RenderError(inner) => inner,
        }
    }

    pub fn as_inner_converted(
        &self,
    ) -> Option<&dyn GtlProjectModuleStateInnerConverted<ProjectModule = ProjectModule>> {
        match self {
            Self::ConvertError(_) => None,
            Self::Converted(inner) => Some(inner),
            Self::ResolveError(inner) => Some(inner),
            Self::Resolved(inner) => Some(inner),
            Self::Rendered(inner) => Some(inner),
            Self::RenderError(inner) => Some(inner),
        }
    }

    pub fn as_inner_converted_mut(
        &mut self,
    ) -> Option<&mut dyn GtlProjectModuleStateInnerConverted<ProjectModule = ProjectModule>> {
        match self {
            Self::ConvertError(_) => None,
            Self::Converted(inner) => Some(inner),
            Self::ResolveError(inner) => Some(inner),
            Self::Resolved(inner) => Some(inner),
            Self::Rendered(inner) => Some(inner),
            Self::RenderError(inner) => Some(inner),
        }
    }

    pub fn as_inner_maybe_resolved(
        &self,
    ) -> Option<&dyn GtlProjectModuleStateInnerMaybeResolved<ProjectModule = ProjectModule>> {
        match self {
            Self::ConvertError(_) | Self::Converted(_) | Self::ResolveError(_) => None,
            Self::Resolved(inner) => Some(inner),
            Self::Rendered(inner) => Some(inner),
            Self::RenderError(inner) => Some(inner),
        }
    }

    pub fn as_inner_resolved(
        &self,
    ) -> Option<&dyn GtlProjectModuleStateInnerResolved<Module = ProjectModule>> {
        match self {
            Self::ConvertError(_) | Self::Converted(_) | Self::ResolveError(_) => None,
            Self::Resolved(inner) => Some(inner),
            Self::Rendered(inner) => inner.resolved().map(|inner| {
                inner as &dyn GtlProjectModuleStateInnerResolved<Module = ProjectModule>
            }),
            Self::RenderError(inner) => inner.resolved().map(|inner| {
                inner as &dyn GtlProjectModuleStateInnerResolved<Module = ProjectModule>
            }),
        }
    }
}

pub trait GtlProjectModuleStateInnerNamed {
    fn name(&self) -> &'static str;
}

pub trait GtlProjectModuleStateInner {
    fn source_path(&self) -> &GtpModulePath;

    fn target_path(&self) -> Option<&GtpTargetFilePath>;
}

pub type GtlProjectModuleStateInnerConvertedProjectModule<StateInner> =
    <StateInner as GtlProjectModuleStateInnerConverted>::ProjectModule;

pub trait GtlProjectModuleStateInnerConverted: GtlProjectModuleStateInner {
    type ProjectModule: GtlProjectModule;

    fn converted(&self) -> &GtlProjectModuleConverted<Self::ProjectModule>;

    fn converted_mut(&mut self) -> &mut GtlProjectModuleConverted<Self::ProjectModule>;

    fn project_module(&self) -> &Self::ProjectModule {
        self.converted().project_module()
    }

    fn project_module_mut(&mut self) -> &mut Self::ProjectModule {
        self.converted_mut().project_module_mut()
    }

    fn dependencies(
        &self,
    ) -> IndexSet<GtlProjectModuleTypeDependencyIdent<'_, Self::ProjectModule>> {
        self.project_module().dependencies()
    }
}

pub trait GtlProjectModuleStateInnerConvertedWrapper: GtlProjectModuleStateInnerConverted {}

impl<Converted: GtlProjectModuleStateInnerConvertedWrapper> GtlProjectModuleStateInner
    for Converted
{
    fn source_path(&self) -> &GtpModulePath {
        self.converted().source_path()
    }

    fn target_path(&self) -> Option<&GtpTargetFilePath> {
        self.converted().target_path()
    }
}

pub trait GtlProjectModuleStateInnerMaybeResolved {
    type ProjectModule: GtlProjectModule;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<Self::ProjectModule>>;
}

pub trait GtlProjectModuleStateInnerResolved {
    type Module: GtlProjectModule;

    fn resolved(&self) -> &GtlProjectModuleResolved<Self::Module>;
}

impl<Resolved: GtlProjectModuleStateInnerResolved> GtlProjectModuleStateInnerMaybeResolved
    for Resolved
{
    type ProjectModule = Resolved::Module;

    fn resolved(&self) -> Option<&GtlProjectModuleResolved<Self::ProjectModule>> {
        Some(self.resolved())
    }
}

pub trait GtlProjectModuleStateInnerToResolveError:
    GtlProjectModuleStateInnerNamed + GtlProjectModuleStateInnerConverted
{
    fn to_resolve_error_state(
        self,
    ) -> GtlProjectModuleState<GtlProjectModuleStateInnerConvertedProjectModule<Self>>
    where
        Self: Sized,
    {
        let source_state_name = self.name().to_owned();
        GtlProjectModuleState::ResolveError(GtlProjectModuleResolveErrorState {
            source_state: self.to_resolve_error_source_state(),
            error: GtlProjectModuleResolveError::State { source_state_name },
        })
    }

    fn to_resolve_error_source_state(
        self,
    ) -> GtlProjectModuleResolveErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    >;
}

pub trait GtlProjectModuleStateInnerToRenderError:
    GtlProjectModuleStateInnerNamed + GtlProjectModuleStateInnerConverted
{
    fn to_render_error_state(
        self,
    ) -> GtlProjectModuleState<GtlProjectModuleStateInnerConvertedProjectModule<Self>>
    where
        Self: Sized,
    {
        let source_state_name = self.name().to_owned();
        GtlProjectModuleState::RenderError(GtlProjectModuleRenderErrorState {
            source_state: self.to_render_error_source_state(),
            error: GtlProjectModuleRenderError::State { source_state_name },
        })
    }

    fn to_render_error_source_state(
        self,
    ) -> GtlProjectModuleRenderErrorSourceState<
        GtlProjectModuleStateInnerConvertedProjectModule<Self>,
    >;
}
