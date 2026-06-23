use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlProjectFileExtra {
    Generated(GtlProjectFileExtraGenerated),
    Error(GtlProjectFileExtraError),
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFileExtra> for GtlProjectFile<ProjectModule> {
    fn from(val: GtlProjectFileExtra) -> Self {
        GtlProjectFile::Extra(val)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFileExtra> for GtlGeneration<ProjectModule> {
    fn from(val: GtlProjectFileExtra) -> Self {
        GtlGeneration::file(GtlProjectFile::Extra(val))
    }
}

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlProjectFileExtraError {
    #[error("Failed to generate `{target_path}`")]
    Generate {
        target_path: GtpTargetFilePath,
        #[source]
        #[diagnostic_source]
        error: Box<dyn GtlError>,
    },
}

impl From<GtlProjectFileExtraError> for GtlProjectFileExtra {
    fn from(val: GtlProjectFileExtraError) -> Self {
        GtlProjectFileExtra::Error(val)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFileExtraError>
    for GtlProjectFile<ProjectModule>
{
    fn from(val: GtlProjectFileExtraError) -> Self {
        GtlProjectFile::Extra(val.into())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlProjectFileExtraGenerated {
    pub path: GtpTargetFilePath,
    pub source_code: String,
}

impl From<GtlProjectFileExtraGenerated> for GtlProjectFileExtra {
    fn from(val: GtlProjectFileExtraGenerated) -> Self {
        GtlProjectFileExtra::Generated(val)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFileExtraGenerated>
    for GtlProjectFile<ProjectModule>
{
    fn from(val: GtlProjectFileExtraGenerated) -> Self {
        GtlProjectFile::Extra(val.into())
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFileExtraGenerated>
    for GtlGeneration<ProjectModule>
{
    fn from(val: GtlProjectFileExtraGenerated) -> Self {
        GtlGeneration::file(GtlProjectFile::Extra(val.into()))
    }
}
