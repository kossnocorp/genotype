use crate::prelude::internal::*;

// region: Modules

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;

mod error;
pub use error::*;

mod source;
pub use source::*;

mod notice;

// endregion

// region: Module

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtpModule {
    /// Module is currently being loaded.
    Initialized(GtpModuleSource),

    /// Module failed to load.
    Error(GtpModuleSource, GtpModuleError),

    /// Module has been parsed successfully.
    Parsed(GtpModuleParse),

    /// Module has been resolved successfully.
    Resolved(GtpModuleResolved),
}

impl GtpModule {
    /// Module state name.
    pub fn name(&self) -> &str {
        match self {
            GtpModule::Initialized(_) => "initialized",
            GtpModule::Error(_, _) => "error",
            GtpModule::Parsed(_) => "parsed",
            GtpModule::Resolved(_) => "resolved",
        }
    }

    /// Resolves the module parse.
    pub fn module_parse(&self) -> Option<&GtModuleParse> {
        self.project_module_parse().map(|parse| &parse.module_parse)
    }

    /// Resolves the project module parse.
    pub fn project_module_parse(&self) -> Option<&GtpModuleParse> {
        match self {
            GtpModule::Initialized(_) => None,
            GtpModule::Error(_, _) => None,
            GtpModule::Parsed(state) => Some(state),
            GtpModule::Resolved(state) => Some(&state.project_module_parse),
        }
    }

    /// Resolves the module path.
    pub fn path(&self) -> &GtpModulePath {
        match self {
            GtpModule::Initialized(source) => source.path(),
            GtpModule::Error(source, _) => source.path(),
            GtpModule::Parsed(state) => &state.path,
            GtpModule::Resolved(state) => &state.project_module_parse.path,
        }
    }

    /// Resolves the module source.
    pub fn source(&self) -> &GtpModuleSource {
        match self {
            GtpModule::Initialized(source) => source,
            GtpModule::Error(source, _) => source,
            GtpModule::Parsed(state) => &state.source,
            GtpModule::Resolved(state) => &state.project_module_parse.source,
        }
    }

    // Resolves the module source code.
    pub fn source_code(&self) -> Option<&str> {
        match self {
            GtpModule::Initialized(_) => None,
            GtpModule::Error(_, _) => None,
            GtpModule::Parsed(state) => Some(&state.source_code),
            GtpModule::Resolved(state) => Some(&state.project_module_parse.source_code),
        }
    }
}

// endregion
