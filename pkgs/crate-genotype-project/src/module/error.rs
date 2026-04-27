use crate::prelude::internal::*;

/// Project module error. Represents errors that can occur during the loading and resolving of
/// a project module.
#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtpModuleError {
    #[error("Failed to init module `{path}`: {message}")]
    Init {
        path: GtpModulePath,
        message: String,
    },

    #[error("Can't read file source for module `{path}`: {message}")]
    Read {
        path: GtpModulePath,
        message: String,
    },

    #[error("Failed to parse module `{path}`: {error}")]
    Parse {
        path: GtpModulePath,
        #[source]
        error: GtParseError,
        source_code: String,
    },

    #[error("Failed to resolve module `{path}`: {error}")]
    Resolve {
        path: GtpModulePath,
        #[source]
        error: GtpError,
    },

    // TODO: Make it so initialized state is module id or path-aware
    #[error("Resolve failed as module is still in initialized state")]
    ResolveInitialized,
}
