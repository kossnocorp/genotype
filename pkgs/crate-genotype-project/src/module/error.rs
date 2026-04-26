use crate::prelude::internal::*;

/// Project module error. Represents errors that can occur during the loading and resolving of
/// a project module.
#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GtpModuleError {
    #[error("Can't read file source for module `{0}`: {1}")]
    Read(GtpModulePath, String),

    #[error("")]
    Parse(#[source] GtParseError),
}
