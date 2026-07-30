use crate::prelude::internal::*;

mod diagnostic;
pub use diagnostic::*;

/// Project module error. Represents errors that can occur during the loading and resolving of
/// a project module.
#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtpModuleError {
    #[error("Failed to init module `{path}`: {message}")]
    Init {
        path: GtpModulePath,
        message: String,
    },

    #[error("Failed to read module source code `{path}`: {message}")]
    Read {
        path: GtpModulePath,
        message: String,
    },

    #[error("Failed to parse module `{path}`: {error}")]
    Parse {
        path: GtpModulePath,
        // #[source]
        // #[diagnostic_source]
        error: GtParseError,
        // #[source_code]
        source_code: String,
    },

    #[error("Failed to resolve module `{path}`: {error}")]
    Resolve {
        path: GtpModulePath,
        #[source]
        error: GtpError,
    },

    #[error("Failed to type check module `{path}`")]
    #[diagnostic(code("asd"))]
    TypeCheck {
        path: GtpModulePath,
        #[source_code]
        source_code: String,
        #[related]
        errors: Vec<GtpModuleTypeCheckError>,
    },

    #[error("Invalid module state: {current_state}, expected: {expected_states}")]
    InvalidModuleState {
        current_state: &'static str,
        expected_states: &'static str,
    },
}
