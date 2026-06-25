use crate::prelude::internal::*;

pub type GtlGenerations<ProjectModule> =
    (Vec<GtlGeneration<ProjectModule>>, Option<Vec<GtDiagnostic>>);

pub struct GtlGeneration<ProjectModule: GtlProjectModule> {
    pub file: GtlProjectFile<ProjectModule>,
    pub diagnostics: Option<Vec<GtDiagnostic>>,
}

impl<ProjectModule: GtlProjectModule> GtlGeneration<ProjectModule> {
    pub fn file<File: Into<GtlProjectFile<ProjectModule>>>(file: File) -> Self {
        GtlGeneration {
            file: file.into(),
            diagnostics: None,
        }
    }

    pub fn file_with_diagnostic<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        diagnostic: GtDiagnostic,
    ) -> Self {
        Self::file_with_diagnostics(file, vec![diagnostic])
    }

    pub fn file_with_diagnostic_option<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        diagnostic: Option<GtDiagnostic>,
    ) -> Self {
        match diagnostic {
            Some(diagnostic) => Self::file_with_diagnostics(file, vec![diagnostic]),
            None => Self::file(file),
        }
    }

    pub fn file_with_diagnostics<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        diagnostics: Vec<GtDiagnostic>,
    ) -> Self {
        GtlGeneration {
            file: file.into(),
            diagnostics: Some(diagnostics),
        }
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, Vec<GtDiagnostic>)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, diagnostics): (GtlProjectFile<ProjectModule>, Vec<GtDiagnostic>)) -> Self {
        Self::file_with_diagnostics(file, diagnostics)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFile<ProjectModule>>
    for GtlGeneration<ProjectModule>
{
    fn from(file: GtlProjectFile<ProjectModule>) -> Self {
        Self::file(file)
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, GtDiagnostic)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, diagnostic): (GtlProjectFile<ProjectModule>, GtDiagnostic)) -> Self {
        Self::file_with_diagnostic(file, diagnostic)
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, Option<GtDiagnostic>)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, diagnostic): (GtlProjectFile<ProjectModule>, Option<GtDiagnostic>)) -> Self {
        Self::file_with_diagnostic_option(file, diagnostic)
    }
}
