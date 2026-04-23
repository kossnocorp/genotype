use crate::prelude::internal::*;

/// Project module error. Represents errors that can occur during the loading and resolving of
/// a project module.
#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtpModuleError {}
