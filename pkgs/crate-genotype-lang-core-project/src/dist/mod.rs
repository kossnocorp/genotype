use crate::prelude::internal::*;

mod file;
pub use file::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDist {
    pub files: Vec<GtlDistFile>,
    pub notices: Vec<GtNotice>,
}

impl GtlDist {
    pub fn pack_modules<ProjectModule: GtlProjectModule>(
        notices: Vec<GtNotice>,
        modules: &IndexMap<GtpModulePath, GtlProjectModuleState<ProjectModule>>,
    ) -> GtlDist {
        let files = modules
            .into_iter()
            .map(|(_module_path, module)| module.into())
            .collect::<Vec<_>>();

        todo!("Convert notices to warnings and errors");

        GtlDist { files, notices }
    }

    pub fn pack_extra_files<ProjectModule: GtlProjectModule>(
        &mut self,
        extra_files: Vec<GtlGeneration<ProjectModule>>,
    ) {
        for extra in extra_files {
            self.files.push(extra.file.into());
            if let Some(notices) = extra.notices {
                self.notices.extend(notices);
            }
        }
    }
}
