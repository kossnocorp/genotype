use crate::prelude::internal::*;

mod file;
pub use file::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDist {
    pub files: Vec<GtlDistFile>,
    pub diagnostics: Vec<GtDiagnostic>,
}

impl GtlDist {
    pub fn new<ProjectModule: GtlProjectModule>(
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<ProjectModule>>,
        diagnostics: Vec<GtDiagnostic>,
    ) -> GtlDist {
        let files = modules.values().map(|module| module.into()).collect();
        GtlDist { files, diagnostics }
    }

    pub fn pack_extra_files<ProjectModule: GtlProjectModule>(
        &mut self,
        extra_files: Vec<GtlGeneration<ProjectModule>>,
        extra_diagnostics: Option<Vec<GtDiagnostic>>,
    ) {
        for extra in extra_files {
            self.files.push(extra.file.into());
            if let Some(diagnostics) = extra.diagnostics {
                self.diagnostics.extend(diagnostics);
            }
        }

        if let Some(extra_diagnostics) = extra_diagnostics {
            self.diagnostics.extend(extra_diagnostics);
        }
    }

    pub fn sort_files(&mut self) {
        self.files.sort_by(|a, b| a.path().cmp(b.path()));
    }
}
