use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlProjectFileExtra {
    Generated(GtlProjectFileExtraGenerated),
    Error(GtlProjectFileExtraError),
}

impl<ProjectModule: GtlProjectModule> Into<GtlProjectFile<ProjectModule>> for GtlProjectFileExtra {
    fn into(self) -> GtlProjectFile<ProjectModule> {
        GtlProjectFile::Extra(self)
    }
}

impl<ProjectModule: GtlProjectModule> Into<GtlGeneration<ProjectModule>> for GtlProjectFileExtra {
    fn into(self) -> GtlGeneration<ProjectModule> {
        GtlGeneration::file(GtlProjectFile::Extra(self))
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

impl Into<GtlProjectFileExtra> for GtlProjectFileExtraError {
    fn into(self) -> GtlProjectFileExtra {
        GtlProjectFileExtra::Error(self)
    }
}

impl<ProjectModule: GtlProjectModule> Into<GtlProjectFile<ProjectModule>>
    for GtlProjectFileExtraError
{
    fn into(self) -> GtlProjectFile<ProjectModule> {
        GtlProjectFile::Extra(self.into())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlProjectFileExtraGenerated {
    pub path: GtpTargetFilePath,
    pub source_code: String,
}

impl Into<GtlProjectFileExtra> for GtlProjectFileExtraGenerated {
    fn into(self) -> GtlProjectFileExtra {
        GtlProjectFileExtra::Generated(self)
    }
}

impl<ProjectModule: GtlProjectModule> Into<GtlProjectFile<ProjectModule>>
    for GtlProjectFileExtraGenerated
{
    fn into(self) -> GtlProjectFile<ProjectModule> {
        GtlProjectFile::Extra(self.into())
    }
}

impl<ProjectModule: GtlProjectModule> Into<GtlGeneration<ProjectModule>>
    for GtlProjectFileExtraGenerated
{
    fn into(self) -> GtlGeneration<ProjectModule> {
        GtlGeneration::file(GtlProjectFile::Extra(self.into()))
    }
}
