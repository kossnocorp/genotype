use crate::prelude::internal::*;

mod file;
pub use file::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDist {
    pub files: Vec<GtlDistFile>,
    pub notices: Vec<GtNotice>,
}

impl GtlDist {
    pub fn new<ProjectModule: GtlProjectModule>(
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<ProjectModule>>,
        notices: Vec<GtNotice>,
    ) -> GtlDist {
        let files = modules.values().map(|module| module.into()).collect();
        GtlDist { files, notices }
    }

    pub fn pack_extra_files<ProjectModule: GtlProjectModule>(
        &mut self,
        extra_files: Vec<GtlGeneration<ProjectModule>>,
        extra_notices: Option<Vec<GtNotice>>,
    ) {
        for extra in extra_files {
            self.files.push(extra.file.into());
            if let Some(notices) = extra.notices {
                self.notices.extend(notices);
            }
        }

        if let Some(extra_notices) = extra_notices {
            self.notices.extend(extra_notices);
        }
    }

    pub fn sort_files(&mut self) {
        self.files.sort_by(|a, b| a.path().cmp(b.path()));
    }
}
