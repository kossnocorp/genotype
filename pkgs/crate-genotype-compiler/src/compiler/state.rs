use crate::prelude::internal::*;
pub enum GtCompilerState {
    New {
        config_path: Option<GtpCwdRelativePath>,
        meta: GtcMetaNew,
    },
    LoadedProject {
        project: GtProject,
        meta: GtcMetaLoadedProject,
    },
    LoadedModules {
        project: GtProject,
        meta: GtcMetaLoadedModules,
    },
    Compiled {
        project: GtProject,
        meta: GtcMetaCompiled,
    },
}
