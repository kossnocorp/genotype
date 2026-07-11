use crate::prelude::internal::*;

mod file;
pub use file::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDist {
    pub files: Vec<GtlDistFile>,
    pub modules: Vec<GtlDistModule>,
    pub diagnostics: Vec<GtDiagnostic>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDistModule {
    pub source_path: GtpModulePath,
    pub target_path: GtpTargetFilePath,
}

impl GtlDist {
    pub fn new<ProjectModule: GtlProjectModule>(
        generations: Vec<GtlGeneration<ProjectModule>>,
        diagnostics: Vec<GtDiagnostic>,
    ) -> GtlDist {
        let mut dist = GtlDist {
            files: vec![],
            modules: vec![],
            diagnostics,
        };
        for generation in generations {
            dist.pack_generation(generation);
        }
        dist
    }

    pub fn pack_extra_files<ProjectModule: GtlProjectModule>(
        &mut self,
        extra_files: Vec<GtlGeneration<ProjectModule>>,
        extra_diagnostics: Option<Vec<GtDiagnostic>>,
    ) {
        for extra in extra_files {
            self.pack_generation(extra);
        }

        if let Some(extra_diagnostics) = extra_diagnostics {
            self.diagnostics.extend(extra_diagnostics);
        }
    }

    pub fn sort_files(&mut self) {
        self.files.sort_by(|a, b| a.path().cmp(b.path()));
    }

    fn pack_generation<ProjectModule: GtlProjectModule>(
        &mut self,
        generation: GtlGeneration<ProjectModule>,
    ) {
        if let GtlProjectFile::Module(GtlProjectModuleState::Rendered(rendered)) = &generation.file
        {
            let converted = rendered.converted();
            self.modules.push(GtlDistModule {
                source_path: converted.source_path.clone(),
                target_path: converted.target_path.clone(),
            });
        }

        self.files.push(generation.file.into());
        if let Some(diagnostics) = generation.diagnostics {
            self.diagnostics.extend(diagnostics);
        }
    }
}
