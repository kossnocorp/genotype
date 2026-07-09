use crate::prelude::internal::*;

pub enum GtCompilerState {
    New {
        config_path: Option<GtpCwdRelativePath>,
    },
    LoadedProject {
        project: GtProject,
    },
    LoadedModules {
        project: GtProject,
    },
}
