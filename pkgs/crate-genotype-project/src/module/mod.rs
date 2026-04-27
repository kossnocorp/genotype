use crate::prelude::internal::*;

// region: Modules

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;

mod error;
pub use error::*;

// endregion

// region: Module

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtpModule {
    /// Module is currently being loaded.
    Initialized,

    /// Module failed to load.
    Error(GtpModuleError),

    /// Module has been parsed successfully.
    Parsed(GtpModuleParse),

    /// Module has been resolved successfully.
    Resolved(GtpModuleResolved),
}

impl GtpModule {
    /// Resolves the module parse.
    pub fn module_parse(&self) -> Option<&GtModuleParse> {
        self.project_module_parse().map(|parse| &parse.module_parse)
    }

    /// Resolves the project module parse.
    pub fn project_module_parse(&self) -> Option<&GtpModuleParse> {
        match self {
            GtpModule::Parsed(state) => Some(&state),
            GtpModule::Resolved(state) => Some(&state.project_module_parse),
            GtpModule::Error(_) => None,
            GtpModule::Initialized => None,
        }
    }
}

// endregion
