use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GtlProjectModuleConvertErrorState {
    pub error: GtlProjectModuleConvertError,
}

impl GtlProjectModuleStateInnerNamed for GtlProjectModuleConvertErrorState {
    fn name(&self) -> &'static str {
        "convert error"
    }
}

impl GtlProjectModuleStateInner for GtlProjectModuleConvertErrorState {
    fn target_path(&self) -> Option<&GtpTargetFilePath> {
        self.error.target_path()
    }

    fn source_path(&self) -> &GtpModulePath {
        self.error.source_path()
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectModuleConvertErrorState>
    for GtlProjectModuleState<ProjectModule>
{
    fn from(val: GtlProjectModuleConvertErrorState) -> Self {
        GtlProjectModuleState::ConvertError(val)
    }
}
